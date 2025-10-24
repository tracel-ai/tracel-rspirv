// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::{
    dr::{self, Builder},
    spirv,
};
impl Builder {
    #[allow(clippy::too_many_arguments)]
    pub fn gl_round(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_round_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_round_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Round as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_round_even(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_round_even_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_round_even_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::RoundEven as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_trunc(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_trunc_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_trunc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Trunc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FAbs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_s_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SAbs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FSign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_s_sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SSign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_floor(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_floor_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_floor_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Floor as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_ceil(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_ceil_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_ceil_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Ceil as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_fract(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_fract_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Fract as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_radians(
        &mut self,
        result_type: spirv::Word,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_radians_id(result_type, None, degrees)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_radians_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(degrees)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Radians as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_degrees(
        &mut self,
        result_type: spirv::Word,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_degrees_id(result_type, None, radians)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_degrees_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(radians)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Degrees as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_asin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_asin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_asin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Asin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_acos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_acos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_acos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Acos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atan(
        &mut self,
        result_type: spirv::Word,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_atan_id(result_type, None, y_over_x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y_over_x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sinh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_sinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cosh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_cosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_tanh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_tanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_tanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Tanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_asinh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_asinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_asinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Asinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_acosh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_acosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_acosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Acosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atanh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_atanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atan2(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_atan2_id(result_type, None, y, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_atan2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atan2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pow(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pow_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pow_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Pow as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_inverse_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_inverse_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_inverse_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InverseSqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_determinant(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_determinant_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_determinant_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Determinant as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_matrix_inverse(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_matrix_inverse_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_matrix_inverse_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::MatrixInverse as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_modf_id(result_type, None, x, i)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(i)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Modf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_modf_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_modf_struct_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_modf_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::ModfStruct as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_u_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_s_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_u_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_s_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_u_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_s_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_f_mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_f_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::FMix as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_i_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_i_mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_i_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::IMix as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_step(
        &mut self,
        result_type: spirv::Word,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_step_id(result_type, None, edge, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(edge), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Step as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_smooth_step(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_smooth_step_id(result_type, None, edge0, edge1, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_smooth_step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::SmoothStep as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_fma(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_fma_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_fma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::Fma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_frexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_frexp_id(result_type, None, x, exp)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_frexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Frexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_frexp_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_frexp_struct_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_frexp_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FrexpStruct as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_ldexp_id(result_type, None, x, exp)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Ldexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_snorm4x8_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackSnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_unorm4x8_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackUnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_snorm2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackSnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_unorm2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackUnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_half2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackHalf2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_pack_double2x32_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_pack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackDouble2x32 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_snorm2x16_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackSnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_unorm2x16_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackUnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_half2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackHalf2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_snorm4x8_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackSnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_unorm4x8_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackUnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_unpack_double2x32_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_unpack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackDouble2x32 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_length(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_length_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Length as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_distance_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cross(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_cross_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cross as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_normalize(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_normalize_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_face_forward(
        &mut self,
        result_type: spirv::Word,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_face_forward_id(result_type, None, n, i, nref)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_face_forward_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(n),
            dr::Operand::IdRef(i),
            dr::Operand::IdRef(nref),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FaceForward as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_reflect(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_reflect_id(result_type, None, i, n)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_reflect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(i), dr::Operand::IdRef(n)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Reflect as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_refract(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_refract_id(result_type, None, i, n, eta)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_refract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(i),
            dr::Operand::IdRef(n),
            dr::Operand::IdRef(eta),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Refract as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_i_lsb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_find_i_lsb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_i_lsb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindILsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_s_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_find_s_msb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_s_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindSMsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_u_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_find_u_msb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_find_u_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindUMsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_centroid(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_interpolate_at_centroid_id(result_type, None, interpolant)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_centroid_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtCentroid as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_sample(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_interpolate_at_sample_id(result_type, None, interpolant, sample)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_sample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant), dr::Operand::IdRef(sample)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtSample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_offset(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_interpolate_at_offset_id(result_type, None, interpolant, offset)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_interpolate_at_offset_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant), dr::Operand::IdRef(offset)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtOffset as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_n_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_n_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.gl_n_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn gl_n_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NClamp as spirv::Word,
            args,
        )
    }
}
