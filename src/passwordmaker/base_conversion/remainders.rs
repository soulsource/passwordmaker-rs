/// Adds `calc_remainders(divisor)` method to types that have some implementation of the Division trait.
pub(super) trait CalcRemainders<T, D>{
    fn calc_remainders(self, divisor : D) -> Remainders<T,D>;
}

/// Implement `Division` to enable the `calc_remainders()` method for your type.
pub(super) trait Division<D> where Self:Sized {
    /// does in-place arbitrary-length division. Returns remainder.
    fn divide(self, divisor : &D) -> DivisionResult<Self, D>;
    fn is_zero(&self) -> bool;
}

/// Or mark your type as `UseGenericDivision` to just use `/` and `%` operators for types. Makes only sense for integers.
pub(super) trait UseGenericDivision : Clone 
    + for <'a> std::ops::Div<&'a Self, Output = Self> 
    + for <'a> std::ops::Rem<&'a Self, Output = Self> 
    + Default
    + Eq {}

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

pub(super) struct DivisionResult<T:Division<U>, U> {
    pub result : T,
    pub remainder : U,
}

impl<U> Division<U> for U
    where U: UseGenericDivision
{
    fn divide(self, divisor : &Self) -> DivisionResult<Self, Self> {
        DivisionResult { 
            result: self.clone().div(divisor),
            remainder: self.rem(divisor)
        }
    }
    fn is_zero(&self) -> bool {
        *self == Self::default()
    }
}