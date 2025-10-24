// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    dr::{self, Builder},
    spirv,
};
impl Builder {
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_acos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acosh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_acosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acospi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_acospi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_acospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acospi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_asin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asinh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_asinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asinpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_asinpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_asinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asinpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_atan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan2(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_atan2_id(result_type, None, y, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atanh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_atanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atanpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_atanpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atanpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan2pi(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_atan2pi_id(result_type, None, y, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_atan2pi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan2pi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cbrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_cbrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cbrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cbrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ceil(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_ceil_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ceil_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ceil as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_copysign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_copysign_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_copysign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::copysign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cosh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_cosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cospi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_cospi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cospi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_erfc(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_erfc_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_erfc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::erfc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_erf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_erf_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_erf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::erf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_expm1(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_expm1_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_expm1_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::expm1 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fabs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fabs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fabs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fabs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fdim(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fdim_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fdim_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fdim as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_floor(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_floor_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_floor_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::floor as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fma(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fma_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmax(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fmax_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmax_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fmin_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmod(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fmod_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmod_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmod as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fract(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fract_id(result_type, None, x, ptr)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(ptr)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fract as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_frexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_frexp_id(result_type, None, x, exp)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_frexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::frexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_hypot(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_hypot_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_hypot_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::hypot as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ilogb(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_ilogb_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ilogb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ilogb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_ldexp_id(result_type, None, x, k)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(k)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ldexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_lgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_lgamma_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_lgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::lgamma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_lgamma_r(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_lgamma_r_id(result_type, None, x, signp)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_lgamma_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(signp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::lgamma_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log1p(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_log1p_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_log1p_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log1p as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_logb(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_logb_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_logb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::logb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_mad(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_mad_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_mad_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::mad as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_maxmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_maxmag_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_maxmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::maxmag as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_minmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_minmag_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_minmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::minmag as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_modf_id(result_type, None, x, iptr)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(iptr)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::modf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_nan(
        &mut self,
        result_type: spirv::Word,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_nan_id(result_type, None, nancode)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_nan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(nancode)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::nan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_nextafter(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_nextafter_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_nextafter_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::nextafter as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_pow(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_pow_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_pow_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::pow as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_pown(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_pown_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_pown_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::pown as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_remainder(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_remainder_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_remainder_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::remainder as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_remquo(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_remquo_id(result_type, None, x, y, quo)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_remquo_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(quo),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::remquo as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rint(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_rint_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rint_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rint as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rootn(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_rootn_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rootn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rootn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_round(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_round_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_round_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::round as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sincos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sincos_id(result_type, None, x, cosval)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sincos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(cosval)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sincos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sinh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sinpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sinpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sinpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tanh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_tanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tanpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_tanpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tanpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_tgamma_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_tgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tgamma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_trunc(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_trunc_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_trunc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::trunc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_divide_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_divide as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_recip_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_recip as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_half_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_half_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_divide_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_divide as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_recip_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_recip as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_native_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_native_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_abs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_abs_diff_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_abs_diff as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_add_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_add_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_add_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_add_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_hadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_hadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_hadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_hadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_rhadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_rhadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_rhadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_rhadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_clamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_clamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_clamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_clamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_clz(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_clz_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_clz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::clz as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ctz(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_ctz_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_ctz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ctz as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_mad_hi_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_mad_sat_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_mad_sat_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_max as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_max as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_min as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_min as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_mul_hi_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mul_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rotate(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_rotate_id(result_type, None, v, i)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_rotate_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v), dr::Operand::IdRef(i)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rotate as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_sub_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_sub_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_sub_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_sub_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_upsample_id(result_type, None, hi, lo)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(hi), dr::Operand::IdRef(lo)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_upsample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_upsample_id(result_type, None, hi, lo)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(hi), dr::Operand::IdRef(lo)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_upsample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_popcount(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_popcount_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_popcount_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::popcount as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_mad24_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_mad24_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_s_mul24_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_s_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mul24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_mul24_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mul24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_abs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_abs_diff_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_abs_diff as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_mul_hi_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mul_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_u_mad_hi_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_u_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fclamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fclamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fclamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fclamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_degrees(
        &mut self,
        result_type: spirv::Word,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_degrees_id(result_type, None, radians)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_degrees_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(radians)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::degrees as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmax_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fmax_common_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmax_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmax_common as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmin_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fmin_common_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fmin_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmin_common as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(a),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::mix as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_radians(
        &mut self,
        result_type: spirv::Word,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_radians_id(result_type, None, degrees)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_radians_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(degrees)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::radians as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_step(
        &mut self,
        result_type: spirv::Word,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_step_id(result_type, None, edge, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(edge), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::step as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_smoothstep(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_smoothstep_id(result_type, None, edge0, edge1, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_smoothstep_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(edge0),
            dr::Operand::IdRef(edge1),
            dr::Operand::IdRef(x),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::smoothstep as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cross(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_cross_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cross as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_distance_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_length_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::length as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_normalize_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fast_distance_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fast_length_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_length as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_fast_normalize_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_fast_normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_bitselect(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_bitselect_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_bitselect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::bitselect as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_select(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_select_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_select_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::select as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vloadn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vloadn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vloadn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vloadn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstoren(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstoren_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstoren_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstoren as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vload_half(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vload_half_id(result_type, None, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vload_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(offset), dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vload_half as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vload_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vload_halfn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vload_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vload_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_half(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstore_half_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_half as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_half_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstore_half_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_half_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_half_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstore_halfn_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstore_halfn_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstore_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_halfn_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vloada_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vloada_halfn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vloada_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vloada_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstorea_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstorea_halfn_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstorea_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstorea_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstorea_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_vstorea_halfn_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_vstorea_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstorea_halfn_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_shuffle(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_shuffle_id(result_type, None, x, shuffle_mask)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_shuffle_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(shuffle_mask)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::shuffle as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_shuffle2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_shuffle2_id(result_type, None, x, y, shuffle_mask)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_shuffle2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(shuffle_mask),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::shuffle2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_printf(
        &mut self,
        result_type: spirv::Word,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_printf_id(result_type, None, format, additional_arguments)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_printf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(format)];
        args.extend(additional_arguments.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::printf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_prefetch(
        &mut self,
        result_type: spirv::Word,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cl_prefetch_id(result_type, None, ptr, num_elements)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn cl_prefetch_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(num_elements)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::prefetch as spirv::Word,
            args,
        )
    }
}
