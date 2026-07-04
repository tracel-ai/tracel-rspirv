use derive_new::new;
use pliron::attribute::AttrObj;

use crate::prelude::*;

pub use crate::autogen_decorations::*;

#[derive(new, Clone, Debug)]
pub struct DecorationInfo {
    pub decoration: Decoration,
    pub value: AttrObj,
}

impl Eq for DecorationInfo {}
impl PartialEq for DecorationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.decoration == other.decoration && self.value.eq_attr(&*other.value)
    }
}
