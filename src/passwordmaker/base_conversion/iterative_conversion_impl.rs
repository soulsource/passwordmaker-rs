//! Implementation of iterative conversion support for the types we need it for: u128 and [u32;N].

//Reminder for myself: The traits needed are:
//    where V: for<'a> From<&'a B> +                    //could be replaced by num::traits::identities::One.
//          for<'a> DivAssign<&'a B> +                  //used between steps to go to next-lower current_base_potency
//          RemAssignWithQuotient+                      //used to get the result of each step.
//          TryInto<B>,                                 //used to convert the result of each step. We _know_ this cannot fail, but requiring Into would be wrong.
//      for<'a> &'a V : Mul<&'a B, Output = Option<V>>  //used to get the first current_base_potency.

//let's start with the simple case: u128
//we do need a NewType here, because actual u128 already has a Mul<&usize> implementation that does not match the version we want.

use std::{ops::{DivAssign, Mul, SubAssign}, convert::{TryFrom, TryInto}, fmt::Display, error::Error, cmp::Ordering};

use super::iterative_conversion::RemAssignWithQuotient;

//Type to be used as V, with usize as B.
pub(crate) struct SixteenBytes(u128);

impl SixteenBytes{
    pub(super) fn new(value : u128) -> Self {
        SixteenBytes(value)
    }
}

//just for convenience
impl From<u128> for SixteenBytes{
    fn from(x: u128) -> Self {
        SixteenBytes(x)
    }
}
impl From<&usize> for SixteenBytes{
    fn from(x: &usize) -> Self {
        SixteenBytes(*x as u128)
    }
}
impl DivAssign<&usize> for SixteenBytes{
    fn div_assign(&mut self, rhs: &usize) {
        self.0 /= *rhs as u128
    }
}
impl RemAssignWithQuotient for SixteenBytes{
    fn rem_assign_with_quotient(&mut self, divisor : &Self) -> Self {
        let quotient = self.0 / divisor.0;
        self.0 %= divisor.0;
        Self(quotient)
    }
}
impl TryFrom<SixteenBytes> for usize{
    type Error = std::num::TryFromIntError;
    fn try_from(value: SixteenBytes) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
impl Mul<&usize> for &SixteenBytes{
    type Output = Option<SixteenBytes>;
    fn mul(self, rhs: &usize) -> Self::Output {
        self.0.checked_mul(*rhs as u128).map(Into::into)
    }
}

//--------------------------------------------------------------------------------------------------------------------------------------
//and now the hard part: The same for [u32;N].
//We cannot directly implement all the Foreign traits on arrays directly. So, newtypes again.

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
pub(crate) struct ArbitraryBytes<const N : usize>([u32;N]);

//Const generics are still a bit limited -> let's just implement From for the exact types we need.
impl From<&usize> for ArbitraryBytes<5>{
    fn from(x: &usize) -> Self {
        Self([
            0,//(*x >> 32*4) as u32, //zero on all target platforms
            0,//(*x >> 32*3) as u32, //zero on all target platforms
            0,//(*x >> 32*2) as u32, //zero on all target platforms
            x.checked_shr(32).map(|x| x as u32).unwrap_or_default(),
            *x as u32,
        ])
    }
}

impl From<&usize> for ArbitraryBytes<8>{
    fn from(x: &usize) -> Self {
        Self([
            0,//(*x >> 32*7) as u32, //zero on all target platforms
            0,//(*x >> 32*6) as u32, //zero on all target platforms
            0,//(*x >> 32*5) as u32, //zero on all target platforms
            0,//(*x >> 32*4) as u32, //zero on all target platforms
            0,//(*x >> 32*3) as u32, //zero on all target platforms
            0,//(*x >> 32*2) as u32, //zero on all target platforms
            x.checked_shr(32).map(|x| x as u32).unwrap_or_default(),
            *x as u32,
        ])
    }
}

impl From<&u32> for ArbitraryBytes<5>{
    fn from(x: &u32) -> Self {
        Self([
            0,
            0,
            0,
            0,
            *x,
        ])
    }
}

impl From<&u32> for ArbitraryBytes<8>{
    fn from(x: &u32) -> Self {
        Self([
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            *x,
        ])
    }
}

//workaround for lack of proper const-generic support.
pub(super) trait PadWithAZero{
    type Output;
    fn pad_with_a_zero(&self) -> Self::Output;
}

impl PadWithAZero for ArbitraryBytes<5>{
    type Output = ArbitraryBytes<6>;
    fn pad_with_a_zero(&self) -> Self::Output {
        ArbitraryBytes::<6>([
            0,
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
        ])
    }
}

impl PadWithAZero for ArbitraryBytes<8>{
    type Output = ArbitraryBytes<9>;
    fn pad_with_a_zero(&self) -> Self::Output {
        ArbitraryBytes::<9>([
            0,
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
        ])
    }
}

impl<const N : usize> DivAssign<&usize> for ArbitraryBytes<N>{
    //just do long division.
    fn div_assign(&mut self, rhs: &usize) {
        self.div_assign_with_remainder_usize(rhs);
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArbitraryBytesToUsizeError;
impl Display for ArbitraryBytesToUsizeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversion from arbitrary sized int-array to usize failed")
    }
}
impl Error for ArbitraryBytesToUsizeError{}

impl<const N : usize> TryFrom<ArbitraryBytes<N>> for usize{
    type Error = ArbitraryBytesToUsizeError;

    fn try_from(value: ArbitraryBytes<N>) -> Result<Self, Self::Error> {
        usize::try_from(&value)
    }
}

impl<const N : usize> TryFrom<&ArbitraryBytes<N>> for usize{
    type Error = ArbitraryBytesToUsizeError;
    #[cfg(target_pointer_width = "64")]
    fn try_from(value: &ArbitraryBytes<N>) -> Result<Self, Self::Error> {
        //64 bits.
        if value.0[0..N.saturating_sub(2)].iter().any(|x| *x != 0) {
            Err(ArbitraryBytesToUsizeError)
        } else {
            //failing to get last_bit is an actual error.
            let last_bit = value.0.get(N-1).ok_or(ArbitraryBytesToUsizeError).map(|x| *x as usize);
            //second-last is not an error though.
            let second_last_bit = value.0.get(N-2).map(|u| (*u as usize) << 32).unwrap_or_default();
            last_bit.map(|last_bit| last_bit + second_last_bit)
        }
    }
    #[cfg(not(target_pointer_width = "64"))]
    fn try_from(value: &ArbitraryBytes<N>) -> Result<Self, Self::Error> {
        //16 or 32 bits.
        if value.0[0..N.saturating_sub(1)].iter().any(|x| *x != 0) {
            Err(ArbitraryBytesToUsizeError)
        } else {
            value.0.get(N-1).and_then(|x| (*x).try_into().ok()).ok_or(ArbitraryBytesToUsizeError)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArbitraryBytesToU32Error;
impl Display for ArbitraryBytesToU32Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversion from arbitrary sized int-array to u32 failed")
    }
}
impl Error for ArbitraryBytesToU32Error{}

impl<const N : usize> TryFrom<&ArbitraryBytes<N>> for u32{
    type Error = ArbitraryBytesToU32Error;

    fn try_from(value: &ArbitraryBytes<N>) -> Result<Self, Self::Error> {
        if value.0[0..N.saturating_sub(1)].iter().any(|x| *x != 0) {
            Err(ArbitraryBytesToU32Error)
        } else {
            value.0.get(N-1).and_then(|x| (*x).try_into().ok()).ok_or(ArbitraryBytesToU32Error)
        }
    }
}

impl<const N : usize> Mul<&usize> for &ArbitraryBytes<N>{
    type Output = Option<ArbitraryBytes<N>>;
    fn mul(self, rhs: &usize) -> Self::Output {
        #[cfg(target_pointer_width = "64")]
        type UsizeAndFour = u128;
        #[cfg(not(target_pointer_width = "64"))]
        type UsizeAndFour = u64;
        //somewhere we need this clone, can just as well be in here...
        let mut result = self.0.clone();
        let carry = result.iter_mut().rev().fold(UsizeAndFour::default(), |carry, digit|{
            debug_assert_eq!(carry, carry & (usize::MAX as UsizeAndFour)); //carry always has to fit in usize, otherwise something is terribly wrong.
            let res = (*digit as UsizeAndFour) * (*rhs as UsizeAndFour) + carry;
            *digit = res as u32;
            res >> 32
        });
        if carry != 0 { //if there's still carry after we hit the last digit, well, didn't fit obviously.
            None
        } else {
            Some(ArbitraryBytes(result))
        }
    }
}

impl<const N : usize, const M : usize> RemAssignWithQuotient for ArbitraryBytes<N> 
    where Self : for<'a> From<&'a usize> + for<'a> From<&'a u32> + PadWithAZero<Output = ArbitraryBytes<M>>
{
    fn rem_assign_with_quotient(&mut self, divisor : &Self) -> Self{

        //This is based on Knuth, TAOCP vol 2 section 4.3, algorithm D. However, at least for now, a 
        //non-performing restoring version of the algorithm is used, because I'm too tired right now
        //to properly implement the performing one (which would with near certainty be faster a bit).
        
        //well, nearly without trying to be smart.
        match Ord::cmp(self, divisor){
            std::cmp::Ordering::Less => Self::from(&0_usize), //leave self unchanged, it's the remainder.
            std::cmp::Ordering::Equal => { *self = Self::from(&0_usize); Self::from(&1_usize) },
            std::cmp::Ordering::Greater => {
                if let Ok(divisor_as_u32) = divisor.try_into() {
                    self.rem_assign_with_quotient_u32(&divisor_as_u32)
                } else {
                    self.rem_assign_with_quotient_knuth(divisor)
                }
            },
        }
    }
}

/// Needed by rem_assign_with_quotient_knuth
impl<const N : usize> Mul<u32> for ArbitraryBytes<N>{
    type Output = Option<ArbitraryBytes<N>>;
    fn mul(mut self, rhs: u32) -> Self::Output {
        let carry = self.0.iter_mut().rev().fold(u64::default(), |carry, digit|{
            debug_assert_eq!(carry, carry & (u32::MAX as u64)); //carry always has to fit in usize, otherwise something is terribly wrong.
            let res = (*digit as u64) * (rhs as u64) + carry;
            *digit = res as u32;
            res >> 32
        });
        if carry != 0 { //if there's still carry after we hit the last digit, well, didn't fit obviously.
            None
        } else {
            Some(self)
        }
    }
}

impl<const N : usize> SubAssign<&ArbitraryBytes<N>> for ArbitraryBytes<N>{
    fn sub_assign(&mut self, rhs: &ArbitraryBytes<N>) {
        let carry = self.0.iter_mut().zip(rhs.0.iter()).rev().fold(0_u64,|carry,(i,s)| {
            let s = (*s as u64) + carry;
            if *i as u64 >= s {
                *i -= s as u32;
                0
            } else {
                *i = (((*i as u64) + (1_u64<<32)) - s) as u32;
                1
            }
        });
        debug_assert_eq!(carry,0);
    }
}


impl<const N : usize> ArbitraryBytes<N>{
    pub(super) fn new(data : [u32;N]) -> Self {
        ArbitraryBytes(data)
    }

    /// Replaces self with Quotient and returns Remainder
    fn div_assign_with_remainder_usize(&mut self, rhs: &usize) -> usize {
        #[cfg(target_pointer_width = "64")]
        type UsizeAndFour = u128;
        #[cfg(not(target_pointer_width = "64"))]
        type UsizeAndFour = u64;
        debug_assert!((UsizeAndFour::MAX >> 32) as u128 >= usize::MAX as u128);

        let divisor : UsizeAndFour = *rhs as UsizeAndFour;
        let remainder = self.0.iter_mut().fold(0 as UsizeAndFour,|carry, current| {
            debug_assert_eq!(carry, carry & (usize::MAX as UsizeAndFour)); //carry has to be lower than divisor, and divisor is usize.
            let carry_shifted = carry << 32;
            let dividend = (carry_shifted) + (*current as UsizeAndFour);
            let ratio = dividend / divisor;
            debug_assert_eq!(ratio, ratio & 0xffff_ffff); //this is fine. The first digit after re-adding the carry is alwys zero.
            *current = (ratio) as u32; 
            dividend - ratio * divisor
        });
        debug_assert_eq!(remainder, remainder & (usize::MAX as UsizeAndFour));
        remainder as usize
    }

    /// Used in rem_assign_with_quotient_knuth. The normalization factor is u32, and u32 might be larger than usize.
    fn div_assign_with_remainder_u32(&mut self, rhs: &u32) -> u32 {
        let divisor : u64 = *rhs as u64;
        let remainder = self.0.iter_mut().fold(0_u64,|carry, current| {
            debug_assert_eq!(carry, carry & (u32::MAX as u64)); //carry has to be lower than divisor, and divisor is usize.
            let carry_shifted = carry << 32;
            let dividend = (carry_shifted) + (*current as u64);
            let ratio = dividend / divisor;
            debug_assert_eq!(ratio, ratio & 0xffff_ffff); //this is fine. The first digit after re-adding the carry is alwys zero.
            *current = (ratio) as u32; 
            dividend - ratio * divisor
        });
        debug_assert_eq!(remainder, remainder & (u32::MAX as u64));
        remainder as u32
    }

    fn rem_assign_with_quotient_u32(&mut self, divisor: &u32) -> Self where Self : for<'a> From<&'a u32> {
        let remainder = self.div_assign_with_remainder_u32(divisor);
        std::mem::replace(self, Self::from(&remainder))
    }
    

    fn rem_assign_with_quotient_knuth<const M : usize>(&mut self, divisor : &Self) -> Self
        where Self : PadWithAZero<Output = ArbitraryBytes<M>> +
                     for<'a> From<&'a usize>
    {
        debug_assert!(M == N+1);
        //first we need to find n (number of digits in divisor)
        let n_digits_divisor= N - divisor.find_first_nonzero_digit();
        debug_assert!(n_digits_divisor > 1);
        //and same in the non-normalized dividend
        let m_plus_n_digits_dividend = N - self.find_first_nonzero_digit();
        let m_extra_digits_dividend = m_plus_n_digits_dividend - n_digits_divisor;

        //step D1: Normalize. This brings the maximum error for each digit down to no more than 2.
        let normalize_shift = divisor.get_digit_from_right(n_digits_divisor - 1).leading_zeros() as usize;
        //again, missing const generics ruin all the fun.
        let mut dividend = self.shift_left(normalize_shift);
        let divisor = divisor.shift_left(normalize_shift);
        debug_assert_eq!(divisor.get_digit_from_right(n_digits_divisor - 1).leading_zeros(),0);

        let mut quotient : Self = (&0_usize).into();

        //needed for Step D3.
        let guess_divisor = divisor.get_digit_from_right(n_digits_divisor - 1) as u64;
        let divisor_second_significant_digit = divisor.get_digit_from_right(n_digits_divisor-2) as u64;

        //step D2, D7: the loop.
        for j in (0..=m_extra_digits_dividend).rev() {
            //Step D3: Guess a digit
            let guess_dividend = ((dividend.get_digit_from_right(j+n_digits_divisor) as u64)<<32) + (dividend.get_digit_from_right(j + n_digits_divisor - 1) as u64);
            let mut guesstimate = guess_dividend/guess_divisor;
            let mut guess_reminder = guess_dividend % guess_divisor;
            //refine this result (still step D3)
            while guess_reminder <= u32::MAX as u64
                && (guesstimate > u32::MAX as u64
                    || divisor_second_significant_digit * guesstimate
                        > (guess_reminder << 32) + (dividend.get_digit_from_right(j + n_digits_divisor - 2) as u64)
                ) {
                guesstimate -= 1;
                guess_reminder += guess_divisor;
            }
            //I'm too tired to do this by the book. If this thing is gonna blow, we can just as well increase our guesstimate by one and call it a day.
            //In any case, this does only happen in _very_ rare cases. Soo:
            //Steps D4-D6.
            debug_assert!(guesstimate & (u32::MAX as u64) == guesstimate); //Knuth says this is a one-place number, and I trust him.
            let mut guesstimate = guesstimate as u32;
            let mut s = (divisor.clone() * guesstimate).expect("Multipliation by a digit cannot overflow for a padded type.");
            let will_overflow = 
                std::cmp::PartialOrd::lt(&dividend.0[(M - 1 - (j+n_digits_divisor))..=(M - 1 - j)], &s.0[(M - 1 - n_digits_divisor)..=(M - 1)]);
            if will_overflow {
                guesstimate -= 1;
                s -= &divisor;
                debug_assert!(std::cmp::Ord::cmp(&dividend.0[(M - 1 - (j+n_digits_divisor))..=(M - 1 - j)], &s.0[(M - 1 - n_digits_divisor)..=(M - 1)]) != Ordering::Less)
            }
            slice_sub_assign(&mut dividend.0[(M - 1 - (j+n_digits_divisor))..=(M - 1 - j)], &s.0[(M - 1 - n_digits_divisor)..=(M - 1)]);
            quotient.set_digit_from_right(guesstimate, j);
        }

        //Steop D8: Compute Remainder.
        self.0 = dividend.shift_right(normalize_shift).0[1..].try_into()
            .expect("Conversion of what should have been an N-element slice into an N-element array failed.");
        quotient
        
    }

    fn find_first_nonzero_digit(&self) -> usize{
        self.0.iter().enumerate().skip_while(|(_,v)| **v == 0).next().map(|(x,_)| x).unwrap_or(N)
    }

    fn get_digit_from_right(&self, i : usize) -> u32{
        self.0[N-i-1]
    }
    fn set_digit_from_right(&mut self, val: u32, i : usize){
        self.0[N-i-1] = val;
    }

    fn shift_left<const M : usize>(&self, s : usize) -> <Self as PadWithAZero>::Output
        where Self : PadWithAZero<Output = ArbitraryBytes<M>>
    {
        debug_assert!(s < 32);
        let mut res = self.pad_with_a_zero();
        if s != 0{
            let _ = res.0.iter_mut().rev().fold(0u32,|carry, val| {
                let c = *val >> (32 - s);
                *val <<= s;
                debug_assert!(*val & carry == 0);
                *val += carry;
                c
            });
        }
        res
    }

    fn shift_right(mut self, s : usize) -> Self {
        debug_assert!(s < 32);
        if s != 0 {
            let _ = self.0.iter_mut().fold(0u32, |carry, val| {
                let c = *val << (32-s);
                *val >>= s;
                debug_assert!(*val & carry == 0);
                *val += carry;
                c
            });
        }
        self
    }
}

fn slice_sub_assign(lhs : &mut [u32], rhs: &[u32]){
    debug_assert_eq!(lhs.len(), rhs.len());
    let carry = lhs.iter_mut().zip(rhs.iter()).rev().fold(0_u64,|carry,(i,s)| {
        let s = (*s as u64) + carry;
        if *i as u64 >= s {
            *i -= s as u32;
            0
        } else {
            *i = (((*i as u64) + (1_u64<<32)) - s) as u32;
            1
        }
    });
    debug_assert_eq!(carry,0);
}

#[cfg(test)]
mod iterative_conversion_impl_tests{
    use super::*;
    use rand::RngCore;
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256Plus;

    /// Tests specifically the case that will_overflow is true.
    #[test]
    fn knuth_add_back_test(){
        let mut dividend = ArbitraryBytes::new([
            //m = 3, n=5
            u32::MAX,
            u32::MAX,
            u32::MAX-1,
            u32::MAX,
            u32::MAX,
            0,
            0,
            3
        ]);
        let divisor = ArbitraryBytes::new([
            0,
            0,
            0,
            0,
            0,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ]);
        let result = dividend.rem_assign_with_quotient(&divisor);
        assert_eq!(dividend.0, [0,0,0,0,0,0,0,2]);
        assert_eq!(result.0, [0,0,0,u32::MAX,u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    }


    fn prepare_many_numbers() -> Vec<(ArbitraryBytes<5>,ArbitraryBytes<5>, u128, u128)>{
        let mut rng = Xoshiro256Plus::seed_from_u64(0);
        let mut res = Vec::new();
        for _i in 0..10000000 {
            let dx = rng.next_u32() % 3 + 2; //at least 2 digits, at max 4 (u128)
            let dy = rng.next_u32() % 3 + 2;
            let ds = dx.min(dy);
            let dl = dx.max(dy);
            let dividendx = [
                0,
                if dl == 4 { rng.next_u32() } else { 0 },
                if dl >=3 { rng.next_u32() } else {0},
                rng.next_u32(),
                rng.next_u32(),
            ];
            let divisorx = [
                0,
                if ds == 4 { rng.next_u32() } else { 0 },
                if ds >=3 { rng.next_u32() } else {0},
                rng.next_u32(),
                rng.next_u32(),
            ];
            let needs_swap = ds == dl && dividendx[5-ds as usize] < divisorx[5-ds as usize];
            let dividend = ArbitraryBytes::new(if needs_swap { divisorx } else {dividendx});
            let divisor = ArbitraryBytes::new(if needs_swap {dividendx} else {divisorx});
            assert!(dividend.ge(&divisor));

            let td = 
                ((dividend.0[1] as u128)<<96)
              + ((dividend.0[2] as u128)<<64)
              + ((dividend.0[3] as u128)<<32)
              + (dividend.0[4] as u128);
            let tn = 
                ((divisor.0[1] as u128)<<96)
              + ((divisor.0[2] as u128)<<64)
              + ((divisor.0[3] as u128)<<32)
              + (divisor.0[4] as u128);


            res.push((dividend, divisor, td/tn, td%tn));
        }
        res
    }

    /// Just tests a bunch of procedurally generated numbers (all within u128 for easy comparison.)
    #[test]
    fn knuth_many_numbers_test() {
        let input = prepare_many_numbers();
        for (mut dividend, divisor, expected_quotient, expexted_remainder) in input {
            let quotient = dividend.rem_assign_with_quotient_knuth(&divisor);
            let remainder = dividend;
            let quotient = ((quotient.0[1] as u128)<<(96)) + ((quotient.0[2] as u128)<<64) + ((quotient.0[3] as u128)<<32) + (quotient.0[4] as u128);
            let remainder = ((remainder.0[1] as u128)<<(96)) + ((remainder.0[2] as u128)<<64) + ((remainder.0[3] as u128)<<32) + (remainder.0[4] as u128);
            assert_eq!(quotient, expected_quotient);
            assert_eq!(remainder, expexted_remainder);
        }
    }
}