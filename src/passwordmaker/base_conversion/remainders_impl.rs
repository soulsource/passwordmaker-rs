use super::division::{Division, UseGenericDivision, DivisionResult};

impl UseGenericDivision for u128{} //for Md4, Md5

impl<const N : usize> Division<usize> for [u32;N] {
    #[allow(clippy::cast_possible_truncation)]
    fn divide(mut self, divisor : &usize) -> DivisionResult<Self, usize> {
        #[cfg(target_pointer_width = "64")]
        type UsizeAndFour = u128;
        #[cfg(not(target_pointer_width = "64"))]
        type UsizeAndFour = u64;
        assert!((UsizeAndFour::MAX >> 32) as u128 >= usize::MAX as u128);

        //uses mutation, because why not? self is owned after all :D
        let divisor : UsizeAndFour = *divisor as UsizeAndFour;
        let remainder = self.iter_mut().fold(0 as UsizeAndFour,|carry, current| {
            assert_eq!(carry, carry & (usize::MAX as UsizeAndFour)); //carry has to be lower than divisor, and divisor is usize.
            let carry_shifted = carry << 32;
            let dividend = (carry_shifted) + (*current as UsizeAndFour);
            let ratio = dividend / divisor;
            assert_eq!(ratio, ratio & 0xffff_ffff); //this is fine. The first digit after re-adding the carry is alwys zero.
            *current = (ratio) as u32; 
            dividend - (*current as UsizeAndFour) * divisor
        });
        assert_eq!(remainder, remainder & (usize::MAX as UsizeAndFour));
        let remainder = remainder as usize;
        DivisionResult{
            result: self,
            remainder,
        }
    }

    fn is_zero(&self) -> bool {
        self.iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod remainders_tests{
    use super::super::remainders::CalcRemainders;

    use super::*;
    #[test]
    fn test_generic_division(){
        let v = 50u128;
        let d = 7u128;
        let DivisionResult{result, remainder}=v.divide(&d);
        assert_eq!(7, result);
        assert_eq!(1, remainder);
    }

    #[test]
    fn test_remainders() {
        //relies on generic division.
        let v = 141u128;
        let d = 3u128;
        let results : Vec<u128> = v.calc_remainders(d).collect();
        assert_eq!(results, vec![0u128,2u128,0u128,2u128,1u128])
    }

    #[test]
    fn test_array_divide() {
        let dividend_int = 0xe7f1ec3a5f35af805407a8a531eefb79u128;
        let dividend = [(dividend_int >> 96) as u32, ((dividend_int >> 64) & 0xffffffff) as u32, ((dividend_int >> 32) & 0xffffffff) as u32, (dividend_int & 0xffffffff) as u32];
        #[cfg(target_pointer_width = "64")]
        let divisor = 1531534813576487;
        #[cfg(not(target_pointer_width = "64"))]
        let divisor = 1531534813;
        let result_int = dividend_int / (divisor as u128);
        let remainder_int = dividend_int % (divisor as u128);
        let result = dividend.divide(&divisor);
        assert_eq!(result.result, [(result_int >> 96) as u32, ((result_int >> 64) & 0xffffffff) as u32, ((result_int >> 32) & 0xffffffff) as u32, (result_int & 0xffffffff) as u32]);
        assert_eq!(remainder_int, result.remainder as u128);
    }

}