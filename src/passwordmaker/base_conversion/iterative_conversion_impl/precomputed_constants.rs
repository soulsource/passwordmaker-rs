use super::const_mul_usize;
use super::ArbitraryBytes;
use super::super::iterative_conversion::PrecomputedMaxPowers;

impl PrecomputedMaxPowers<usize> for ArbitraryBytes<5>{
    fn lookup(base : &usize) -> Option<(Self, usize)> { 
        get_from_cache(*base, &CONSTANT_MAX_POWER_CACHE_5)
    }
}

impl PrecomputedMaxPowers<usize> for ArbitraryBytes<8>{
    fn lookup(base : &usize) -> Option<(Self, usize)> { 
        get_from_cache(*base, &CONSTANT_MAX_POWER_CACHE_8)
     }
}

fn get_from_cache<const N : usize>(base : usize, cache : &[([u32;N], usize)]) -> Option<(ArbitraryBytes<N>, usize)>{
    base.checked_sub(2).and_then(|idx|cache.get(idx))
        .map(|c| (ArbitraryBytes(c.0), c.1))
}

const CONSTANT_MAX_POWER_CACHE_5 : [([u32;5],usize);128] = gen_const_max_power_cache();
const CONSTANT_MAX_POWER_CACHE_8 : [([u32;8],usize);128] = gen_const_max_power_cache();

//-----------------------------------------------------------------------------------------

/// This version of `find_highest_fitting_power` is not optimized. But it can run in const contexts. Only use it there, use the normal one everywhere else.
const fn const_find_highest_fitting_power<const N : usize>(base : usize) -> ([u32;N],usize){
    let start = super::from_usize(base);

    let mut x = (start, 1);
    while let Some(next) = const_mul_usize(const_clone(&x.0),base) {
        x.0 = next;
        x.1 +=1;
    }
    (x.0.0, x.1)
}

//If anyone could tell me how to implement "~const Clone" in stable Rust, I'd be very happy.
const fn const_clone<const N : usize>(x : &ArbitraryBytes<N>) -> ArbitraryBytes<N>{ArbitraryBytes(x.0)}

const fn gen_const_max_power_cache<const N : usize, const CNT : usize>() -> [([u32;N],usize);CNT]{
    let mut result = [([0u32;N],0usize);CNT];
    let mut i = 0usize;
    loop {
        let highest = const_find_highest_fitting_power(i + 2);
        result[i] = (highest.0, highest.1);
        i +=1;
        if i == CNT { break; }
    }
    result
}

#[cfg(test)]
mod iterative_conversion_constants_tests{
    use super::ArbitraryBytes;

    #[test]
    fn test_overlows_8()
    {
        let entries = super::CONSTANT_MAX_POWER_CACHE_8.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, power, _exponent) in entries {
            assert!((power * base).is_none())
        }
    }
    #[test]
    fn test_overlows_5()
    {
        let entries = super::CONSTANT_MAX_POWER_CACHE_5.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, power, _exponent) in entries {
            assert!((power * base).is_none())
        }
    }
    #[test]
    fn test_exponent_8()
    {
        let entries = super::CONSTANT_MAX_POWER_CACHE_8.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, mut power, exponent) in entries {
            //exponent is the largest fitting exponent. Soo, if we divide exponent times, we should end up with 1.
            for _i in 0..exponent  {
                let remainder = power.div_assign_with_remainder_usize(base);
                assert_eq!(remainder, 0);
            }
            assert_eq!(power, (&1usize).into());
        }
    }
    #[test]
    fn test_exponent_5()
    {
        let entries = super::CONSTANT_MAX_POWER_CACHE_5.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, mut power, exponent) in entries {
            //exponent is the largest fitting exponent. Soo, if we divide exponent times, we should end up with 1.
            for _i in 0..exponent  {
                let remainder = power.div_assign_with_remainder_usize(base);
                assert_eq!(remainder, 0);
            }
            assert_eq!(power, (&1usize).into());
        }
    }
    #[test]
    fn highest_fitting_power_consistency_5(){
        use super::super::super::iterative_conversion::IterativeBaseConversion;
        let entries = super::CONSTANT_MAX_POWER_CACHE_5.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, power, exponent) in entries {
            let non_cached_result = IterativeBaseConversion::<ArbitraryBytes<5>,usize>::find_highest_fitting_power_non_cached(&base);
            assert_eq!(non_cached_result.exponent,exponent);
            assert_eq!(non_cached_result.power, power);
        }
    }
    #[test]
    fn highest_fitting_power_consistency_8(){
        use super::super::super::iterative_conversion::IterativeBaseConversion;
        let entries = super::CONSTANT_MAX_POWER_CACHE_8.iter().enumerate()
            .map(|(i,(p,e))| (i+2, ArbitraryBytes(*p), *e));
        for (base, power, exponent) in entries {
            let non_cached_result = IterativeBaseConversion::<ArbitraryBytes<8>,usize>::find_highest_fitting_power_non_cached(&base);
            assert_eq!(non_cached_result.exponent,exponent);
            assert_eq!(non_cached_result.power, power);
        }
    }
}