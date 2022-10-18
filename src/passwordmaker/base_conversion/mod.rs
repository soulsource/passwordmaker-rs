use std::convert::TryInto;
use iterative_conversion_impl::PadWithAZero;
pub(super) use iterative_conversion::IterativeBaseConversion;
pub(super) use iterative_conversion_impl::{SixteenBytes, ArbitraryBytes};

mod iterative_conversion;
mod iterative_conversion_impl;

/// Converts an input to a different base (which fits in usize). Returns the digits starting at the most significant one.
pub(super) trait BaseConversion {
    type Output : ExactSizeIterator<Item=usize>;
    // return type is subject to change. Hopefully soon the math will be rewritten, so we can skip the Vec and IntoIter.
    // will have to remain an ExactSizeIterator though.
    fn convert_to_base(self, base : usize) -> Self::Output;
}

impl<T, const N : usize, const M : usize> BaseConversion for T 
    where T : ToArbitraryBytes<Output = ArbitraryBytes<N>>,
        for<'a> T::Output: From<&'a usize> + From<&'a u32> + PadWithAZero<Output = ArbitraryBytes<M>>,
{
    type Output = IterativeBaseConversion<ArbitraryBytes<N>, usize>;
    fn convert_to_base(self, base : usize) -> Self::Output {
        IterativeBaseConversion::new(self.to_arbitrary_bytes(), base)
    }
}

impl BaseConversion for [u8;16]{
    type Output = IterativeBaseConversion<SixteenBytes,usize>;
    fn convert_to_base(self, base : usize) -> IterativeBaseConversion<SixteenBytes,usize> {
        IterativeBaseConversion::new(SixteenBytes::new(u128::from_be_bytes(self)), base)
    }
}



// Rust 1.52 only has a very limited support for const generics. This means, we'll have to live with this not-too-constrained solution...
pub(super) trait ToArbitraryBytes {
    type Output;
    fn to_arbitrary_bytes(self) -> Self::Output;
}

//this could of course be done in a generic manner, but it's ugly without array_mut, which we don't have in Rust 1.52.
//Soo, pedestrian's approach :D 
impl ToArbitraryBytes for [u8;20] {
    type Output = ArbitraryBytes<5>;
    fn to_arbitrary_bytes(self) -> ArbitraryBytes<5> {
        ArbitraryBytes::new([
            u32::from_be_bytes(self[0..4].try_into().unwrap()),
            u32::from_be_bytes(self[4..8].try_into().unwrap()),
            u32::from_be_bytes(self[8..12].try_into().unwrap()),
            u32::from_be_bytes(self[12..16].try_into().unwrap()),
            u32::from_be_bytes(self[16..20].try_into().unwrap()),
        ])
    }
}

impl ToArbitraryBytes for [u8;32] {
    type Output = ArbitraryBytes<8>;
    fn to_arbitrary_bytes(self) -> ArbitraryBytes<8> {
        ArbitraryBytes::new([
            u32::from_be_bytes(self[0..4].try_into().unwrap()),
            u32::from_be_bytes(self[4..8].try_into().unwrap()),
            u32::from_be_bytes(self[8..12].try_into().unwrap()),
            u32::from_be_bytes(self[12..16].try_into().unwrap()),
            u32::from_be_bytes(self[16..20].try_into().unwrap()),
            u32::from_be_bytes(self[20..24].try_into().unwrap()),
            u32::from_be_bytes(self[24..28].try_into().unwrap()),
            u32::from_be_bytes(self[28..32].try_into().unwrap()),
        ])
    }
}