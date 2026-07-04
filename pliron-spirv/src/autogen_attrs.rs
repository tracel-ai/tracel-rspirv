// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use tracel_rspirv::spirv::*;
use crate::prelude::*;
#[pliron_attr(name = "spirv.image_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageOperandsAttr(pub ImageOperands);
impl ::pliron::printable::Printable for ImageOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ImageOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ImageOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ImageOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ImageOperandsAttr {
    pub fn new(value: ImageOperands) -> Self {
        Self(value)
    }
}
impl From<ImageOperands> for ImageOperandsAttr {
    fn from(value: ImageOperands) -> Self {
        Self(value)
    }
}
impl From<ImageOperandsAttr> for ImageOperands {
    fn from(value: ImageOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_fast_math_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPFastMathModeAttr(pub FPFastMathMode);
impl ::pliron::printable::Printable for FPFastMathModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FPFastMathModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FPFastMathMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FPFastMathModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FPFastMathModeAttr {
    pub fn new(value: FPFastMathMode) -> Self {
        Self(value)
    }
}
impl From<FPFastMathMode> for FPFastMathModeAttr {
    fn from(value: FPFastMathMode) -> Self {
        Self(value)
    }
}
impl From<FPFastMathModeAttr> for FPFastMathMode {
    fn from(value: FPFastMathModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.selection_control", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SelectionControlAttr(pub SelectionControl);
impl ::pliron::printable::Printable for SelectionControlAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for SelectionControlAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = SelectionControl::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = SelectionControlAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl SelectionControlAttr {
    pub fn new(value: SelectionControl) -> Self {
        Self(value)
    }
}
impl From<SelectionControl> for SelectionControlAttr {
    fn from(value: SelectionControl) -> Self {
        Self(value)
    }
}
impl From<SelectionControlAttr> for SelectionControl {
    fn from(value: SelectionControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.loop_control", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LoopControlAttr(pub LoopControl);
impl ::pliron::printable::Printable for LoopControlAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for LoopControlAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = LoopControl::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = LoopControlAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl LoopControlAttr {
    pub fn new(value: LoopControl) -> Self {
        Self(value)
    }
}
impl From<LoopControl> for LoopControlAttr {
    fn from(value: LoopControl) -> Self {
        Self(value)
    }
}
impl From<LoopControlAttr> for LoopControl {
    fn from(value: LoopControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.function_control", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FunctionControlAttr(pub FunctionControl);
impl ::pliron::printable::Printable for FunctionControlAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FunctionControlAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FunctionControl::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FunctionControlAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FunctionControlAttr {
    pub fn new(value: FunctionControl) -> Self {
        Self(value)
    }
}
impl From<FunctionControl> for FunctionControlAttr {
    fn from(value: FunctionControl) -> Self {
        Self(value)
    }
}
impl From<FunctionControlAttr> for FunctionControl {
    fn from(value: FunctionControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_semantics", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemorySemanticsAttr(pub MemorySemantics);
impl ::pliron::printable::Printable for MemorySemanticsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for MemorySemanticsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = MemorySemantics::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = MemorySemanticsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl MemorySemanticsAttr {
    pub fn new(value: MemorySemantics) -> Self {
        Self(value)
    }
}
impl From<MemorySemantics> for MemorySemanticsAttr {
    fn from(value: MemorySemantics) -> Self {
        Self(value)
    }
}
impl From<MemorySemanticsAttr> for MemorySemantics {
    fn from(value: MemorySemanticsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_access", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemoryAccessAttr(pub MemoryAccess);
impl ::pliron::printable::Printable for MemoryAccessAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for MemoryAccessAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = MemoryAccess::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = MemoryAccessAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl MemoryAccessAttr {
    pub fn new(value: MemoryAccess) -> Self {
        Self(value)
    }
}
impl From<MemoryAccess> for MemoryAccessAttr {
    fn from(value: MemoryAccess) -> Self {
        Self(value)
    }
}
impl From<MemoryAccessAttr> for MemoryAccess {
    fn from(value: MemoryAccessAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.kernel_profiling_info", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct KernelProfilingInfoAttr(pub KernelProfilingInfo);
impl ::pliron::printable::Printable for KernelProfilingInfoAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for KernelProfilingInfoAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = KernelProfilingInfo::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = KernelProfilingInfoAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl KernelProfilingInfoAttr {
    pub fn new(value: KernelProfilingInfo) -> Self {
        Self(value)
    }
}
impl From<KernelProfilingInfo> for KernelProfilingInfoAttr {
    fn from(value: KernelProfilingInfo) -> Self {
        Self(value)
    }
}
impl From<KernelProfilingInfoAttr> for KernelProfilingInfo {
    fn from(value: KernelProfilingInfoAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_flags", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayFlagsAttr(pub RayFlags);
impl ::pliron::printable::Printable for RayFlagsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for RayFlagsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = RayFlags::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = RayFlagsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl RayFlagsAttr {
    pub fn new(value: RayFlags) -> Self {
        Self(value)
    }
}
impl From<RayFlags> for RayFlagsAttr {
    fn from(value: RayFlags) -> Self {
        Self(value)
    }
}
impl From<RayFlagsAttr> for RayFlags {
    fn from(value: RayFlagsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fragment_shading_rate", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FragmentShadingRateAttr(pub FragmentShadingRate);
impl ::pliron::printable::Printable for FragmentShadingRateAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FragmentShadingRateAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FragmentShadingRate::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FragmentShadingRateAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FragmentShadingRateAttr {
    pub fn new(value: FragmentShadingRate) -> Self {
        Self(value)
    }
}
impl From<FragmentShadingRate> for FragmentShadingRateAttr {
    fn from(value: FragmentShadingRate) -> Self {
        Self(value)
    }
}
impl From<FragmentShadingRateAttr> for FragmentShadingRate {
    fn from(value: FragmentShadingRateAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.raw_access_chain_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RawAccessChainOperandsAttr(pub RawAccessChainOperands);
impl ::pliron::printable::Printable for RawAccessChainOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for RawAccessChainOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = RawAccessChainOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = RawAccessChainOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl RawAccessChainOperandsAttr {
    pub fn new(value: RawAccessChainOperands) -> Self {
        Self(value)
    }
}
impl From<RawAccessChainOperands> for RawAccessChainOperandsAttr {
    fn from(value: RawAccessChainOperands) -> Self {
        Self(value)
    }
}
impl From<RawAccessChainOperandsAttr> for RawAccessChainOperands {
    fn from(value: RawAccessChainOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.source_language", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SourceLanguageAttr(pub SourceLanguage);
impl ::pliron::printable::Printable for SourceLanguageAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for SourceLanguageAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = SourceLanguage::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = SourceLanguageAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl SourceLanguageAttr {
    pub fn new(value: SourceLanguage) -> Self {
        Self(value)
    }
}
impl From<SourceLanguage> for SourceLanguageAttr {
    fn from(value: SourceLanguage) -> Self {
        Self(value)
    }
}
impl From<SourceLanguageAttr> for SourceLanguage {
    fn from(value: SourceLanguageAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.execution_model", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ExecutionModelAttr(pub ExecutionModel);
impl ::pliron::printable::Printable for ExecutionModelAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ExecutionModelAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ExecutionModel::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ExecutionModelAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ExecutionModelAttr {
    pub fn new(value: ExecutionModel) -> Self {
        Self(value)
    }
}
impl From<ExecutionModel> for ExecutionModelAttr {
    fn from(value: ExecutionModel) -> Self {
        Self(value)
    }
}
impl From<ExecutionModelAttr> for ExecutionModel {
    fn from(value: ExecutionModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.addressing_model", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct AddressingModelAttr(pub AddressingModel);
impl ::pliron::printable::Printable for AddressingModelAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for AddressingModelAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = AddressingModel::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = AddressingModelAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl AddressingModelAttr {
    pub fn new(value: AddressingModel) -> Self {
        Self(value)
    }
}
impl From<AddressingModel> for AddressingModelAttr {
    fn from(value: AddressingModel) -> Self {
        Self(value)
    }
}
impl From<AddressingModelAttr> for AddressingModel {
    fn from(value: AddressingModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.memory_model", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MemoryModelAttr(pub MemoryModel);
impl ::pliron::printable::Printable for MemoryModelAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for MemoryModelAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = MemoryModel::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = MemoryModelAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl MemoryModelAttr {
    pub fn new(value: MemoryModel) -> Self {
        Self(value)
    }
}
impl From<MemoryModel> for MemoryModelAttr {
    fn from(value: MemoryModel) -> Self {
        Self(value)
    }
}
impl From<MemoryModelAttr> for MemoryModel {
    fn from(value: MemoryModelAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.execution_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ExecutionModeAttr(pub ExecutionMode);
impl ::pliron::printable::Printable for ExecutionModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ExecutionModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ExecutionMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ExecutionModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ExecutionModeAttr {
    pub fn new(value: ExecutionMode) -> Self {
        Self(value)
    }
}
impl From<ExecutionMode> for ExecutionModeAttr {
    fn from(value: ExecutionMode) -> Self {
        Self(value)
    }
}
impl From<ExecutionModeAttr> for ExecutionMode {
    fn from(value: ExecutionModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.storage_class", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct StorageClassAttr(pub StorageClass);
impl ::pliron::printable::Printable for StorageClassAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for StorageClassAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = StorageClass::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = StorageClassAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl StorageClassAttr {
    pub fn new(value: StorageClass) -> Self {
        Self(value)
    }
}
impl From<StorageClass> for StorageClassAttr {
    fn from(value: StorageClass) -> Self {
        Self(value)
    }
}
impl From<StorageClassAttr> for StorageClass {
    fn from(value: StorageClassAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.dim", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct DimAttr(pub Dim);
impl ::pliron::printable::Printable for DimAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for DimAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = Dim::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = DimAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl DimAttr {
    pub fn new(value: Dim) -> Self {
        Self(value)
    }
}
impl From<Dim> for DimAttr {
    fn from(value: Dim) -> Self {
        Self(value)
    }
}
impl From<DimAttr> for Dim {
    fn from(value: DimAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.sampler_addressing_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SamplerAddressingModeAttr(pub SamplerAddressingMode);
impl ::pliron::printable::Printable for SamplerAddressingModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for SamplerAddressingModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = SamplerAddressingMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = SamplerAddressingModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl SamplerAddressingModeAttr {
    pub fn new(value: SamplerAddressingMode) -> Self {
        Self(value)
    }
}
impl From<SamplerAddressingMode> for SamplerAddressingModeAttr {
    fn from(value: SamplerAddressingMode) -> Self {
        Self(value)
    }
}
impl From<SamplerAddressingModeAttr> for SamplerAddressingMode {
    fn from(value: SamplerAddressingModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.sampler_filter_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct SamplerFilterModeAttr(pub SamplerFilterMode);
impl ::pliron::printable::Printable for SamplerFilterModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for SamplerFilterModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = SamplerFilterMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = SamplerFilterModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl SamplerFilterModeAttr {
    pub fn new(value: SamplerFilterMode) -> Self {
        Self(value)
    }
}
impl From<SamplerFilterMode> for SamplerFilterModeAttr {
    fn from(value: SamplerFilterMode) -> Self {
        Self(value)
    }
}
impl From<SamplerFilterModeAttr> for SamplerFilterMode {
    fn from(value: SamplerFilterModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_format", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageFormatAttr(pub ImageFormat);
impl ::pliron::printable::Printable for ImageFormatAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ImageFormatAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ImageFormat::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ImageFormatAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ImageFormatAttr {
    pub fn new(value: ImageFormat) -> Self {
        Self(value)
    }
}
impl From<ImageFormat> for ImageFormatAttr {
    fn from(value: ImageFormat) -> Self {
        Self(value)
    }
}
impl From<ImageFormatAttr> for ImageFormat {
    fn from(value: ImageFormatAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_channel_order", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageChannelOrderAttr(pub ImageChannelOrder);
impl ::pliron::printable::Printable for ImageChannelOrderAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ImageChannelOrderAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ImageChannelOrder::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ImageChannelOrderAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ImageChannelOrderAttr {
    pub fn new(value: ImageChannelOrder) -> Self {
        Self(value)
    }
}
impl From<ImageChannelOrder> for ImageChannelOrderAttr {
    fn from(value: ImageChannelOrder) -> Self {
        Self(value)
    }
}
impl From<ImageChannelOrderAttr> for ImageChannelOrder {
    fn from(value: ImageChannelOrderAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.image_channel_data_type", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ImageChannelDataTypeAttr(pub ImageChannelDataType);
impl ::pliron::printable::Printable for ImageChannelDataTypeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ImageChannelDataTypeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ImageChannelDataType::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ImageChannelDataTypeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ImageChannelDataTypeAttr {
    pub fn new(value: ImageChannelDataType) -> Self {
        Self(value)
    }
}
impl From<ImageChannelDataType> for ImageChannelDataTypeAttr {
    fn from(value: ImageChannelDataType) -> Self {
        Self(value)
    }
}
impl From<ImageChannelDataTypeAttr> for ImageChannelDataType {
    fn from(value: ImageChannelDataTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_rounding_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPRoundingModeAttr(pub FPRoundingMode);
impl ::pliron::printable::Printable for FPRoundingModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FPRoundingModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FPRoundingMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FPRoundingModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FPRoundingModeAttr {
    pub fn new(value: FPRoundingMode) -> Self {
        Self(value)
    }
}
impl From<FPRoundingMode> for FPRoundingModeAttr {
    fn from(value: FPRoundingMode) -> Self {
        Self(value)
    }
}
impl From<FPRoundingModeAttr> for FPRoundingMode {
    fn from(value: FPRoundingModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_denorm_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPDenormModeAttr(pub FPDenormMode);
impl ::pliron::printable::Printable for FPDenormModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FPDenormModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FPDenormMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FPDenormModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FPDenormModeAttr {
    pub fn new(value: FPDenormMode) -> Self {
        Self(value)
    }
}
impl From<FPDenormMode> for FPDenormModeAttr {
    fn from(value: FPDenormMode) -> Self {
        Self(value)
    }
}
impl From<FPDenormModeAttr> for FPDenormMode {
    fn from(value: FPDenormModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.quantization_modes", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct QuantizationModesAttr(pub QuantizationModes);
impl ::pliron::printable::Printable for QuantizationModesAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for QuantizationModesAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = QuantizationModes::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = QuantizationModesAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl QuantizationModesAttr {
    pub fn new(value: QuantizationModes) -> Self {
        Self(value)
    }
}
impl From<QuantizationModes> for QuantizationModesAttr {
    fn from(value: QuantizationModes) -> Self {
        Self(value)
    }
}
impl From<QuantizationModesAttr> for QuantizationModes {
    fn from(value: QuantizationModesAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_operation_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPOperationModeAttr(pub FPOperationMode);
impl ::pliron::printable::Printable for FPOperationModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FPOperationModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FPOperationMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FPOperationModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FPOperationModeAttr {
    pub fn new(value: FPOperationMode) -> Self {
        Self(value)
    }
}
impl From<FPOperationMode> for FPOperationModeAttr {
    fn from(value: FPOperationMode) -> Self {
        Self(value)
    }
}
impl From<FPOperationModeAttr> for FPOperationMode {
    fn from(value: FPOperationModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.overflow_modes", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct OverflowModesAttr(pub OverflowModes);
impl ::pliron::printable::Printable for OverflowModesAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for OverflowModesAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = OverflowModes::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = OverflowModesAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl OverflowModesAttr {
    pub fn new(value: OverflowModes) -> Self {
        Self(value)
    }
}
impl From<OverflowModes> for OverflowModesAttr {
    fn from(value: OverflowModes) -> Self {
        Self(value)
    }
}
impl From<OverflowModesAttr> for OverflowModes {
    fn from(value: OverflowModesAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.linkage_type", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LinkageTypeAttr(pub LinkageType);
impl ::pliron::printable::Printable for LinkageTypeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for LinkageTypeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = LinkageType::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = LinkageTypeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl LinkageTypeAttr {
    pub fn new(value: LinkageType) -> Self {
        Self(value)
    }
}
impl From<LinkageType> for LinkageTypeAttr {
    fn from(value: LinkageType) -> Self {
        Self(value)
    }
}
impl From<LinkageTypeAttr> for LinkageType {
    fn from(value: LinkageTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.access_qualifier", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct AccessQualifierAttr(pub AccessQualifier);
impl ::pliron::printable::Printable for AccessQualifierAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for AccessQualifierAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = AccessQualifier::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = AccessQualifierAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl AccessQualifierAttr {
    pub fn new(value: AccessQualifier) -> Self {
        Self(value)
    }
}
impl From<AccessQualifier> for AccessQualifierAttr {
    fn from(value: AccessQualifier) -> Self {
        Self(value)
    }
}
impl From<AccessQualifierAttr> for AccessQualifier {
    fn from(value: AccessQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.host_access_qualifier", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct HostAccessQualifierAttr(pub HostAccessQualifier);
impl ::pliron::printable::Printable for HostAccessQualifierAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for HostAccessQualifierAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = HostAccessQualifier::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = HostAccessQualifierAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl HostAccessQualifierAttr {
    pub fn new(value: HostAccessQualifier) -> Self {
        Self(value)
    }
}
impl From<HostAccessQualifier> for HostAccessQualifierAttr {
    fn from(value: HostAccessQualifier) -> Self {
        Self(value)
    }
}
impl From<HostAccessQualifierAttr> for HostAccessQualifier {
    fn from(value: HostAccessQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.function_parameter_attribute", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FunctionParameterAttributeAttr(pub FunctionParameterAttribute);
impl ::pliron::printable::Printable for FunctionParameterAttributeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FunctionParameterAttributeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FunctionParameterAttribute::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FunctionParameterAttributeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FunctionParameterAttributeAttr {
    pub fn new(value: FunctionParameterAttribute) -> Self {
        Self(value)
    }
}
impl From<FunctionParameterAttribute> for FunctionParameterAttributeAttr {
    fn from(value: FunctionParameterAttribute) -> Self {
        Self(value)
    }
}
impl From<FunctionParameterAttributeAttr> for FunctionParameterAttribute {
    fn from(value: FunctionParameterAttributeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.built_in", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct BuiltInAttr(pub BuiltIn);
impl ::pliron::printable::Printable for BuiltInAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for BuiltInAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = BuiltIn::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = BuiltInAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl BuiltInAttr {
    pub fn new(value: BuiltIn) -> Self {
        Self(value)
    }
}
impl From<BuiltIn> for BuiltInAttr {
    fn from(value: BuiltIn) -> Self {
        Self(value)
    }
}
impl From<BuiltInAttr> for BuiltIn {
    fn from(value: BuiltInAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.scope", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ScopeAttr(pub Scope);
impl ::pliron::printable::Printable for ScopeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ScopeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = Scope::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ScopeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ScopeAttr {
    pub fn new(value: Scope) -> Self {
        Self(value)
    }
}
impl From<Scope> for ScopeAttr {
    fn from(value: Scope) -> Self {
        Self(value)
    }
}
impl From<ScopeAttr> for Scope {
    fn from(value: ScopeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.group_operation", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct GroupOperationAttr(pub GroupOperation);
impl ::pliron::printable::Printable for GroupOperationAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for GroupOperationAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = GroupOperation::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = GroupOperationAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl GroupOperationAttr {
    pub fn new(value: GroupOperation) -> Self {
        Self(value)
    }
}
impl From<GroupOperation> for GroupOperationAttr {
    fn from(value: GroupOperation) -> Self {
        Self(value)
    }
}
impl From<GroupOperationAttr> for GroupOperation {
    fn from(value: GroupOperationAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.kernel_enqueue_flags", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct KernelEnqueueFlagsAttr(pub KernelEnqueueFlags);
impl ::pliron::printable::Printable for KernelEnqueueFlagsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for KernelEnqueueFlagsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = KernelEnqueueFlags::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = KernelEnqueueFlagsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl KernelEnqueueFlagsAttr {
    pub fn new(value: KernelEnqueueFlags) -> Self {
        Self(value)
    }
}
impl From<KernelEnqueueFlags> for KernelEnqueueFlagsAttr {
    fn from(value: KernelEnqueueFlags) -> Self {
        Self(value)
    }
}
impl From<KernelEnqueueFlagsAttr> for KernelEnqueueFlags {
    fn from(value: KernelEnqueueFlagsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.capability", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CapabilityAttr(pub Capability);
impl ::pliron::printable::Printable for CapabilityAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CapabilityAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = Capability::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CapabilityAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CapabilityAttr {
    pub fn new(value: Capability) -> Self {
        Self(value)
    }
}
impl From<Capability> for CapabilityAttr {
    fn from(value: Capability) -> Self {
        Self(value)
    }
}
impl From<CapabilityAttr> for Capability {
    fn from(value: CapabilityAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_query_intersection", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryIntersectionAttr(pub RayQueryIntersection);
impl ::pliron::printable::Printable for RayQueryIntersectionAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for RayQueryIntersectionAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = RayQueryIntersection::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = RayQueryIntersectionAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl RayQueryIntersectionAttr {
    pub fn new(value: RayQueryIntersection) -> Self {
        Self(value)
    }
}
impl From<RayQueryIntersection> for RayQueryIntersectionAttr {
    fn from(value: RayQueryIntersection) -> Self {
        Self(value)
    }
}
impl From<RayQueryIntersectionAttr> for RayQueryIntersection {
    fn from(value: RayQueryIntersectionAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_query_committed_intersection_type", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryCommittedIntersectionTypeAttr(pub RayQueryCommittedIntersectionType);
impl ::pliron::printable::Printable for RayQueryCommittedIntersectionTypeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for RayQueryCommittedIntersectionTypeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = RayQueryCommittedIntersectionType::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = RayQueryCommittedIntersectionTypeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl RayQueryCommittedIntersectionTypeAttr {
    pub fn new(value: RayQueryCommittedIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCommittedIntersectionType> for RayQueryCommittedIntersectionTypeAttr {
    fn from(value: RayQueryCommittedIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCommittedIntersectionTypeAttr> for RayQueryCommittedIntersectionType {
    fn from(value: RayQueryCommittedIntersectionTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.ray_query_candidate_intersection_type", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct RayQueryCandidateIntersectionTypeAttr(pub RayQueryCandidateIntersectionType);
impl ::pliron::printable::Printable for RayQueryCandidateIntersectionTypeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for RayQueryCandidateIntersectionTypeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = RayQueryCandidateIntersectionType::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = RayQueryCandidateIntersectionTypeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl RayQueryCandidateIntersectionTypeAttr {
    pub fn new(value: RayQueryCandidateIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCandidateIntersectionType> for RayQueryCandidateIntersectionTypeAttr {
    fn from(value: RayQueryCandidateIntersectionType) -> Self {
        Self(value)
    }
}
impl From<RayQueryCandidateIntersectionTypeAttr> for RayQueryCandidateIntersectionType {
    fn from(value: RayQueryCandidateIntersectionTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.packed_vector_format", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct PackedVectorFormatAttr(pub PackedVectorFormat);
impl ::pliron::printable::Printable for PackedVectorFormatAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for PackedVectorFormatAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = PackedVectorFormat::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = PackedVectorFormatAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl PackedVectorFormatAttr {
    pub fn new(value: PackedVectorFormat) -> Self {
        Self(value)
    }
}
impl From<PackedVectorFormat> for PackedVectorFormatAttr {
    fn from(value: PackedVectorFormat) -> Self {
        Self(value)
    }
}
impl From<PackedVectorFormatAttr> for PackedVectorFormat {
    fn from(value: PackedVectorFormatAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixOperandsAttr(pub CooperativeMatrixOperands);
impl ::pliron::printable::Printable for CooperativeMatrixOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CooperativeMatrixOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = CooperativeMatrixOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CooperativeMatrixOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CooperativeMatrixOperandsAttr {
    pub fn new(value: CooperativeMatrixOperands) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixOperands> for CooperativeMatrixOperandsAttr {
    fn from(value: CooperativeMatrixOperands) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixOperandsAttr> for CooperativeMatrixOperands {
    fn from(value: CooperativeMatrixOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_layout", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixLayoutAttr(pub CooperativeMatrixLayout);
impl ::pliron::printable::Printable for CooperativeMatrixLayoutAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CooperativeMatrixLayoutAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = CooperativeMatrixLayout::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CooperativeMatrixLayoutAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CooperativeMatrixLayoutAttr {
    pub fn new(value: CooperativeMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixLayout> for CooperativeMatrixLayoutAttr {
    fn from(value: CooperativeMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixLayoutAttr> for CooperativeMatrixLayout {
    fn from(value: CooperativeMatrixLayoutAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_use", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixUseAttr(pub CooperativeMatrixUse);
impl ::pliron::printable::Printable for CooperativeMatrixUseAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CooperativeMatrixUseAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = CooperativeMatrixUse::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CooperativeMatrixUseAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CooperativeMatrixUseAttr {
    pub fn new(value: CooperativeMatrixUse) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixUse> for CooperativeMatrixUseAttr {
    fn from(value: CooperativeMatrixUse) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixUseAttr> for CooperativeMatrixUse {
    fn from(value: CooperativeMatrixUseAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_matrix_reduce", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeMatrixReduceAttr(pub CooperativeMatrixReduce);
impl ::pliron::printable::Printable for CooperativeMatrixReduceAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CooperativeMatrixReduceAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = CooperativeMatrixReduce::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CooperativeMatrixReduceAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CooperativeMatrixReduceAttr {
    pub fn new(value: CooperativeMatrixReduce) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixReduce> for CooperativeMatrixReduceAttr {
    fn from(value: CooperativeMatrixReduce) -> Self {
        Self(value)
    }
}
impl From<CooperativeMatrixReduceAttr> for CooperativeMatrixReduce {
    fn from(value: CooperativeMatrixReduceAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_clamp_mode", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorClampModeAttr(pub TensorClampMode);
impl ::pliron::printable::Printable for TensorClampModeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for TensorClampModeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = TensorClampMode::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = TensorClampModeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl TensorClampModeAttr {
    pub fn new(value: TensorClampMode) -> Self {
        Self(value)
    }
}
impl From<TensorClampMode> for TensorClampModeAttr {
    fn from(value: TensorClampMode) -> Self {
        Self(value)
    }
}
impl From<TensorClampModeAttr> for TensorClampMode {
    fn from(value: TensorClampModeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_addressing_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorAddressingOperandsAttr(pub TensorAddressingOperands);
impl ::pliron::printable::Printable for TensorAddressingOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for TensorAddressingOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = TensorAddressingOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = TensorAddressingOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl TensorAddressingOperandsAttr {
    pub fn new(value: TensorAddressingOperands) -> Self {
        Self(value)
    }
}
impl From<TensorAddressingOperands> for TensorAddressingOperandsAttr {
    fn from(value: TensorAddressingOperands) -> Self {
        Self(value)
    }
}
impl From<TensorAddressingOperandsAttr> for TensorAddressingOperands {
    fn from(value: TensorAddressingOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.initialization_mode_qualifier", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct InitializationModeQualifierAttr(pub InitializationModeQualifier);
impl ::pliron::printable::Printable for InitializationModeQualifierAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for InitializationModeQualifierAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = InitializationModeQualifier::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = InitializationModeQualifierAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl InitializationModeQualifierAttr {
    pub fn new(value: InitializationModeQualifier) -> Self {
        Self(value)
    }
}
impl From<InitializationModeQualifier> for InitializationModeQualifierAttr {
    fn from(value: InitializationModeQualifier) -> Self {
        Self(value)
    }
}
impl From<InitializationModeQualifierAttr> for InitializationModeQualifier {
    fn from(value: InitializationModeQualifierAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.load_cache_control", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct LoadCacheControlAttr(pub LoadCacheControl);
impl ::pliron::printable::Printable for LoadCacheControlAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for LoadCacheControlAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = LoadCacheControl::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = LoadCacheControlAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl LoadCacheControlAttr {
    pub fn new(value: LoadCacheControl) -> Self {
        Self(value)
    }
}
impl From<LoadCacheControl> for LoadCacheControlAttr {
    fn from(value: LoadCacheControl) -> Self {
        Self(value)
    }
}
impl From<LoadCacheControlAttr> for LoadCacheControl {
    fn from(value: LoadCacheControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.store_cache_control", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct StoreCacheControlAttr(pub StoreCacheControl);
impl ::pliron::printable::Printable for StoreCacheControlAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for StoreCacheControlAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = StoreCacheControl::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = StoreCacheControlAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl StoreCacheControlAttr {
    pub fn new(value: StoreCacheControl) -> Self {
        Self(value)
    }
}
impl From<StoreCacheControl> for StoreCacheControlAttr {
    fn from(value: StoreCacheControl) -> Self {
        Self(value)
    }
}
impl From<StoreCacheControlAttr> for StoreCacheControl {
    fn from(value: StoreCacheControlAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.named_maximum_number_of_registers", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct NamedMaximumNumberOfRegistersAttr(pub NamedMaximumNumberOfRegisters);
impl ::pliron::printable::Printable for NamedMaximumNumberOfRegistersAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for NamedMaximumNumberOfRegistersAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = NamedMaximumNumberOfRegisters::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = NamedMaximumNumberOfRegistersAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl NamedMaximumNumberOfRegistersAttr {
    pub fn new(value: NamedMaximumNumberOfRegisters) -> Self {
        Self(value)
    }
}
impl From<NamedMaximumNumberOfRegisters> for NamedMaximumNumberOfRegistersAttr {
    fn from(value: NamedMaximumNumberOfRegisters) -> Self {
        Self(value)
    }
}
impl From<NamedMaximumNumberOfRegistersAttr> for NamedMaximumNumberOfRegisters {
    fn from(value: NamedMaximumNumberOfRegistersAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.matrix_multiply_accumulate_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct MatrixMultiplyAccumulateOperandsAttr(pub MatrixMultiplyAccumulateOperands);
impl ::pliron::printable::Printable for MatrixMultiplyAccumulateOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for MatrixMultiplyAccumulateOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = MatrixMultiplyAccumulateOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = MatrixMultiplyAccumulateOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl MatrixMultiplyAccumulateOperandsAttr {
    pub fn new(value: MatrixMultiplyAccumulateOperands) -> Self {
        Self(value)
    }
}
impl From<MatrixMultiplyAccumulateOperands> for MatrixMultiplyAccumulateOperandsAttr {
    fn from(value: MatrixMultiplyAccumulateOperands) -> Self {
        Self(value)
    }
}
impl From<MatrixMultiplyAccumulateOperandsAttr> for MatrixMultiplyAccumulateOperands {
    fn from(value: MatrixMultiplyAccumulateOperandsAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.fp_encoding", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct FPEncodingAttr(pub FPEncoding);
impl ::pliron::printable::Printable for FPEncodingAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for FPEncodingAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = FPEncoding::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = FPEncodingAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl FPEncodingAttr {
    pub fn new(value: FPEncoding) -> Self {
        Self(value)
    }
}
impl From<FPEncoding> for FPEncodingAttr {
    fn from(value: FPEncoding) -> Self {
        Self(value)
    }
}
impl From<FPEncodingAttr> for FPEncoding {
    fn from(value: FPEncodingAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.cooperative_vector_matrix_layout", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct CooperativeVectorMatrixLayoutAttr(pub CooperativeVectorMatrixLayout);
impl ::pliron::printable::Printable for CooperativeVectorMatrixLayoutAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for CooperativeVectorMatrixLayoutAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = CooperativeVectorMatrixLayout::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = CooperativeVectorMatrixLayoutAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl CooperativeVectorMatrixLayoutAttr {
    pub fn new(value: CooperativeVectorMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeVectorMatrixLayout> for CooperativeVectorMatrixLayoutAttr {
    fn from(value: CooperativeVectorMatrixLayout) -> Self {
        Self(value)
    }
}
impl From<CooperativeVectorMatrixLayoutAttr> for CooperativeVectorMatrixLayout {
    fn from(value: CooperativeVectorMatrixLayoutAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.component_type", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct ComponentTypeAttr(pub ComponentType);
impl ::pliron::printable::Printable for ComponentTypeAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for ComponentTypeAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = ComponentType::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = ComponentTypeAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl ComponentTypeAttr {
    pub fn new(value: ComponentType) -> Self {
        Self(value)
    }
}
impl From<ComponentType> for ComponentTypeAttr {
    fn from(value: ComponentType) -> Self {
        Self(value)
    }
}
impl From<ComponentTypeAttr> for ComponentType {
    fn from(value: ComponentTypeAttr) -> Self {
        value.0
    }
}
#[pliron_attr(name = "spirv.tensor_operands", verifier = "succ")]
#[derive(PartialEq, Clone, Debug)]
pub struct TensorOperandsAttr(pub TensorOperands);
impl ::pliron::printable::Printable for TensorOperandsAttr {
    fn fmt(
        &self,
        ctx: &::pliron::context::Context,
        state: &::pliron::printable::State,
        fmt: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        ::pliron::printable::Printable::fmt(&"(", ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&self.0, ctx, state, fmt)?;
        ::pliron::printable::Printable::fmt(&")", ctx, state, fmt)?;
        Ok(())
    }
}
impl ::pliron::parsable::Parsable for TensorOperandsAttr {
    type Arg = ();
    type Parsed = Self;
    fn parse<'__pliron_parse>(
        state_stream: &mut ::pliron::parsable::StateStream<'__pliron_parse>,
        arg: Self::Arg,
    ) -> ::pliron::parsable::ParseResult<'__pliron_parse, Self::Parsed> {
        use ::pliron::parsable::IntoParseResult;
        use ::pliron::combine::Parser;
        use ::pliron::input_err;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string("("))
            .parse_stream(state_stream)
            .into_result()?;
        let field_at_0 = TensorOperands::parse(state_stream, ())?.0;
        ::pliron::irfmt::parsers::spaced(::pliron::combine::parser::char::string(")"))
            .parse_stream(state_stream)
            .into_result()?;
        let final_ret_value = TensorOperandsAttr(field_at_0);
        Ok(final_ret_value).into_parse_result()
    }
    fn parser<'a>(
        _arg: Self::Arg,
    ) -> alloc::boxed::Box<
        dyn ::pliron::combine::Parser<
            ::pliron::parsable::StateStream<'a>,
            Output = Self::Parsed,
            PartialState = (),
        > + 'a,
    > {
        todo!()
    }
}
impl TensorOperandsAttr {
    pub fn new(value: TensorOperands) -> Self {
        Self(value)
    }
}
impl From<TensorOperands> for TensorOperandsAttr {
    fn from(value: TensorOperands) -> Self {
        Self(value)
    }
}
impl From<TensorOperandsAttr> for TensorOperands {
    fn from(value: TensorOperandsAttr) -> Self {
        value.0
    }
}
