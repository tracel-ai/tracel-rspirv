use tracel_rspirv::spirv::{TensorClampMode, Word};

use crate::{ToSpirvType, interfaces::VerCapExtTypeInterface, prelude::*, types::u32_ty};

#[pliron_type(
    name = "spirv.tensor_layout",
    format = "`<` $dim `d, ` $clamp_mode `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TensorLayoutType {
    pub dim: u32,
    pub clamp_mode: TensorClampMode,
}

#[type_interface_impl]
impl ToSpirvType for TensorLayoutType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let dim_id = builder.constant_bit32(ctx, u32_ty(ctx), self.dim)?;
        let clamp_mode_id = builder.constant_bit32(ctx, u32_ty(ctx), self.clamp_mode as u32)?;
        Ok(builder.type_tensor_layout_nv(dim_id, clamp_mode_id))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for TensorLayoutType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, _ctx: &Context) -> Vec<Vec<&'static str>> {
        Operand::from(self.clamp_mode).required_extensions()
    }
    fn required_capabilities(&self, _ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![vec![Capability::TensorAddressingNV]];
        out.extend(Operand::from(self.clamp_mode).required_capabilities());
        out
    }
}

#[pliron_type(
    name = "spirv.tensor_view",
    format = "`<` $dim `d, has_dimensions: ` $has_dimensions `, permutation: [` vec($permutation, CharSpace(`,`)) `] `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TensorViewType {
    pub dim: u32,
    pub has_dimensions: bool,
    pub permutation: Vec<u32>,
}

#[type_interface_impl]
impl ToSpirvType for TensorViewType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let bool = builder.type_bool();
        let dim_id = builder.constant_bit32(ctx, u32_ty(ctx), self.dim)?;
        let has_dims = match self.has_dimensions {
            true => builder.constant_true(bool),
            false => builder.constant_false(bool),
        };
        let permutation = self
            .permutation
            .iter()
            .map(|dim| builder.constant_bit32(ctx, u32_ty(ctx), *dim))
            .collect::<Result<Vec<_>>>()?;
        Ok(builder.type_tensor_view_nv(dim_id, has_dims, permutation))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for TensorViewType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, _ctx: &Context) -> Vec<Vec<&'static str>> {
        vec![]
    }
    fn required_capabilities(&self, _ctx: &Context) -> Vec<Vec<Capability>> {
        vec![vec![Capability::TensorAddressingNV]]
    }
}
