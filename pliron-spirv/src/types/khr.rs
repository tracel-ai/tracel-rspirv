use pliron::r#type::type_cast;
use tracel_rspirv::spirv::{CooperativeMatrixUse, FPEncoding, Scope, Word};

use crate::{
    ToSpirvType,
    interfaces::VerCapExtTypeInterface,
    prelude::*,
    types::{FloatType, u32_ty},
};

#[pliron_type(
    name = "spirv.cooperative_matrix",
    format = "`<` $rows ` x ` $columns ` x ` $component_type `, ` $use_ `, ` $scope `>`",
    generate_get = true,
    verifier = "succ"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CooperativeMatrixType {
    pub component_type: TypeHandle,
    pub scope: Scope,
    pub rows: u32,
    pub columns: u32,
    pub use_: CooperativeMatrixUse,
}

#[type_interface_impl]
impl ToSpirvType for CooperativeMatrixType {
    fn to_spirv(&self, ctx: &Context, builder: &mut PlironBuilder) -> Result<Word> {
        let component_id = spirv_type_id(ctx, builder, self.component_type)?;
        let scope_id = builder.constant_bit32(ctx, u32_ty(ctx), self.scope as u32)?;
        let rows_id = builder.constant_bit32(ctx, u32_ty(ctx), self.rows)?;
        let cols_id = builder.constant_bit32(ctx, u32_ty(ctx), self.columns)?;
        let use_id = builder.constant_bit32(ctx, u32_ty(ctx), self.use_ as u32)?;
        Ok(builder.type_cooperative_matrix_khr(component_id, scope_id, rows_id, cols_id, use_id))
    }
}

#[type_interface_impl]
impl VerCapExtTypeInterface for CooperativeMatrixType {
    fn min_version(&self, _ctx: &Context) -> Option<(u8, u8)> {
        None
    }
    fn required_extensions(&self, ctx: &Context) -> Vec<Vec<&'static str>> {
        let mut out = vec![];
        let component_ty = self.component_type.deref(ctx);
        if let Some(ver_cap_ext) = type_cast::<dyn VerCapExtTypeInterface>(&*component_ty) {
            out.extend(ver_cap_ext.required_extensions(ctx))
        }
        out.extend(Operand::from(self.scope).required_extensions());
        out.extend(Operand::from(self.use_).required_extensions());
        out
    }
    fn required_capabilities(&self, ctx: &Context) -> Vec<Vec<Capability>> {
        let mut out = vec![vec![Capability::CooperativeMatrixKHR]];
        let component_ty = self.component_type.deref(ctx);
        if let Some(ver_cap_ext) = type_cast::<dyn VerCapExtTypeInterface>(&*component_ty) {
            out.extend(ver_cap_ext.required_capabilities(ctx))
        }
        if let Some(float) = component_ty.downcast_ref::<FloatType>() {
            match float.encoding {
                Some(FPEncoding::BFloat16KHR) => out.push(vec![Capability::BFloat16CooperativeMatrixKHR]),
                Some(FPEncoding::Float8E4M3EXT) | Some(FPEncoding::Float8E5M2EXT) => {
                    out.push(vec![Capability::Float8CooperativeMatrixEXT])
                }
                _ => {}
            }
        }
        out.extend(Operand::from(self.scope).required_capabilities());
        out.extend(Operand::from(self.use_).required_capabilities());
        out
    }
}
