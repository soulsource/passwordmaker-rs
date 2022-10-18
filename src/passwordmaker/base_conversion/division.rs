/// A trait that combines std::ops::Div and std::ops::Rem, as those can often be computed together.
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

pub(super) struct DivisionResult<T, U> {
    pub result : T,
    pub remainder : U,
}