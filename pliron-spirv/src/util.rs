use alloc::vec::Vec;

pub(crate) trait ZeroOrMoreValues {
    fn extend_into(self, vec: &mut Vec<Value>);
}

impl ZeroOrMoreValues for Value {
    fn extend_into(self, vec: &mut Vec<Value>) {
        vec.push(self);
    }
}

impl ZeroOrMoreValues for Option<Value> {
    fn extend_into(self, vec: &mut Vec<Value>) {
        vec.extend(self);
    }
}

impl ZeroOrMoreValues for Vec<Value> {
    fn extend_into(self, vec: &mut Vec<Value>) {
        vec.extend(self);
    }
}

macro_rules! flat_vec {
    ($($items: expr),*) => {{
        #[allow(unused_mut)]
        let mut out = alloc::vec::Vec::new();
        $($crate::util::ZeroOrMoreValues::extend_into($items, &mut out);)*
        out
    }};
}
pub(crate) use flat_vec;
use pliron::value::Value;
