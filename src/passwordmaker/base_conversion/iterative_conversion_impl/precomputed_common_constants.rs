//! Precomputed max fitting powers and exponents for common password character lits.
//! 10 is "digits only"
//! 16 is "hexadecimal"
//! 32 is "special characters only"
//! 52 is "letters only"
//! 62 is "letters and digits"
//! 94 is "letters, digits and special characters" - the default for PasswordMaker Pro.

use super::super::iterative_conversion::PrecomputedMaxPowers;
use super::ArbitraryBytes;

impl PrecomputedMaxPowers<usize> for ArbitraryBytes<5>{
    fn lookup(base : &usize) -> Option<(Self, usize)> { 
        match base {
            10 => Some((ArbitraryBytes([0xAF29_8D05, 0x0E43_95D6, 0x9670_B12B, 0x7F41_0000, 0x0000_0000]), 48)),
            16 => Some((ArbitraryBytes([0x1000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000]), 39)),
            32 => Some((ArbitraryBytes([0x0800_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000]), 31)),
            52 => Some((ArbitraryBytes([0xC3AC_AD73, 0xBB2B_01F7, 0x6D5D_11C1, 0xF100_0000, 0x0000_0000]), 28)),
            62 => Some((ArbitraryBytes([0x0702_2C89, 0x3992_DDB9, 0xC9B6_E9D6, 0x5CE5_4443, 0x0400_0000]), 26)),
            94 => Some((ArbitraryBytes([0x27AC_9E29, 0x5D2F_DF56, 0x4DA2_58BA, 0x7B1F_542F, 0x8100_0000]), 24)),
            _ => None
        }
    }
}

impl PrecomputedMaxPowers<usize> for ArbitraryBytes<8>{
    fn lookup(base : &usize) -> Option<(Self, usize)> { 
        match base {
            10 => Some((ArbitraryBytes([0xDD15_FE86, 0xAFFA_D912, 0x49EF_0EB7, 0x13F3_9EBE, 0xAA98_7B6E, 0x6FD2_A000, 0x0000_0000, 0x0000_0000]), 77)),
            16 => Some((ArbitraryBytes([0x1000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000]), 63)),
            32 => Some((ArbitraryBytes([0x8000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000]), 51)),
            52 => Some((ArbitraryBytes([0x070E_F55B, 0x69EB_9498, 0x3F55_F32D, 0x0BB1_D645, 0x1D6E_AA22, 0x3100_0000, 0x0000_0000, 0x0000_0000]), 44)),
            62 => Some((ArbitraryBytes([0x0437_92AD, 0xB7D6_D494, 0xD37D_50A9, 0xCA83_391F, 0x58DB_8150, 0x3744_EF95, 0x05BB_0400, 0x0000_0000]), 42)),
            94 => Some((ArbitraryBytes([0xC5F2_400A, 0x64FC_C0E8, 0x33E1_BCF0, 0x9749_C06B, 0xF160_B863, 0x83C3_ACB8, 0xEC85_2780, 0x0000_0000]), 39)),
            _ => None
        }
     }
}

#[cfg(test)]
mod precomputed_common_constants_tests{
    use super::super::super::PrecomputedMaxPowers;
    use super::super::super::ArbitraryBytes;
    use super::super::super::iterative_conversion::IterativeBaseConversion;

    #[test]
    fn highest_fitting_power_consistency_5(){
        let mut count = 0;
        for base in 2..200 {
            if let Some(precomputed) = ArbitraryBytes::<5>::lookup(&base) {
                let non_cached_result = IterativeBaseConversion::<ArbitraryBytes<5>,usize>::find_highest_fitting_power_non_cached(&base);
                assert_eq!(non_cached_result.exponent, precomputed.1);
                assert_eq!(non_cached_result.power, precomputed.0);
                count += 1;
            }
        }
        assert!(count > 0);
    }
    #[test]
    fn highest_fitting_power_consistency_8(){
        let mut count = 0;
        for base in 2..200 {
            if let Some(precomputed) = ArbitraryBytes::<8>::lookup(&base) {
                let non_cached_result = IterativeBaseConversion::<ArbitraryBytes<8>,usize>::find_highest_fitting_power_non_cached(&base);
                assert_eq!(non_cached_result.exponent, precomputed.1);
                assert_eq!(non_cached_result.power, precomputed.0);
                count += 1;
            }
        }
        assert!(count > 0);
    }
}