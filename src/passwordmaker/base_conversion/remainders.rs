use super::division::{Division, DivisionResult};

/// Trait used for the old base conversion. 
pub(super) trait CalcRemainders<T, D>{
    fn calc_remainders(self, divisor : D) -> Remainders<T,D>;
}

impl<T, D> CalcRemainders<T, D> for T 
    where T:Division<D>
{
    fn calc_remainders(self, divisor : D) -> Remainders<T, D> {
        Remainders::new(self,divisor)
    }
}

pub(super) struct Remainders<T, U>{
    value : Option<T>,
    divisor : U,
}

impl<U, T:Division<U>> Remainders<T, U> {
    fn new(value : T, divisor : U) -> Self {
        let value = if value.is_zero() { None } else { Some(value) };
        Remainders {
            value,
            divisor,
        }
    }
}

impl<U, T:Division<U>> Iterator for Remainders<T,U>{
    type Item=U;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.value.take() {
            let DivisionResult{result, remainder} = v.divide(&self.divisor);
            self.value = if result.is_zero() { None } else { Some(result) };
            Some(remainder)
        } else {
            None
        }
    }
}