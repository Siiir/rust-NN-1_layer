use num_traits::PrimInt;

pub trait IntExpect {
    type ProvidedInt: PrimInt;
    type BoolExpectation: crate::util::BoolExpect;
    fn is_met_by(&self, value: Self::ProvidedInt) -> bool;
    fn bit_expectation(&self, idx: usize) -> Self::BoolExpectation;
}
impl<I: PrimInt> IntExpect for I {
    type ProvidedInt = I;
    type BoolExpectation = bool;

    fn is_met_by(&self, value: Self::ProvidedInt) -> bool {
        *self == value
    }

    fn bit_expectation(&self, idx: usize) -> Self::BoolExpectation {
        !(*self << idx).is_zero()
    }
}
