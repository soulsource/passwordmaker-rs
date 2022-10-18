use std::convert::TryInto;

use remainders::CalcRemainders;

mod division;
mod iterative_conversion;
mod iterative_conversion_impl;

mod remainders;
mod remainders_impl;

/// Converts an input to a different base (which fits in usize). Returns the digits starting at the most significant one.
pub(super) trait BaseConversion {
    // return type is subject to change. Hopefully soon the math will be rewritten, so we can skip the Vec and IntoIter.
    // will have to remain an ExactSizeIterator though.
    fn convert_to_base(self, base : usize) -> std::iter::Rev<std::vec::IntoIter<usize>>;
}

impl<T, const N : usize> BaseConversion for T where T : ToI32Array<Output = [u32;N]>{
    fn convert_to_base(self, base : usize) -> std::iter::Rev<std::vec::IntoIter<usize>> {
        self.to_int_array().calc_remainders(base).collect::<Vec<_>>().into_iter().rev()
    }
}

impl BaseConversion for [u8;16]{
    fn convert_to_base(self, base : usize) -> std::iter::Rev<std::vec::IntoIter<usize>> {
        u128::from_be_bytes(self).calc_remainders(base as u128).map(|ll| ll as usize).collect::<Vec<_>>().into_iter().rev()
    }
}



// Rust 1.52 only has a very limited support for const generics. This means, we'll have to live with this not-too-constrained solution...
pub(super) trait ToI32Array {
    type Output;
    fn to_int_array(self) -> Self::Output;
}

//this could of course be done in a generic manner, but it's ugly without array_mut, which we don't have in Rust 1.52.
//Soo, pedestrian's approach :D 
impl ToI32Array for [u8;20] {
    type Output = [u32; 5];
    fn to_int_array(self) -> [u32; 5] {
        [
            u32::from_be_bytes(self[0..4].try_into().unwrap()),
            u32::from_be_bytes(self[4..8].try_into().unwrap()),
            u32::from_be_bytes(self[8..12].try_into().unwrap()),
            u32::from_be_bytes(self[12..16].try_into().unwrap()),
            u32::from_be_bytes(self[16..20].try_into().unwrap()),
        ]
    }
}

impl ToI32Array for [u8;32] {
    type Output = [u32; 8];
    fn to_int_array(self) -> [u32; 8] {
        [
            u32::from_be_bytes(self[0..4].try_into().unwrap()),
            u32::from_be_bytes(self[4..8].try_into().unwrap()),
            u32::from_be_bytes(self[8..12].try_into().unwrap()),
            u32::from_be_bytes(self[12..16].try_into().unwrap()),
            u32::from_be_bytes(self[16..20].try_into().unwrap()),
            u32::from_be_bytes(self[20..24].try_into().unwrap()),
            u32::from_be_bytes(self[24..28].try_into().unwrap()),
            u32::from_be_bytes(self[28..32].try_into().unwrap()),
        ]
    }
}