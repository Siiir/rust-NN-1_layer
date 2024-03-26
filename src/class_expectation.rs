use crate::util::{BoolExpect, BoolExpectation, IntExpect};
use derive_more::From;

#[derive(From, Clone, Copy, Debug)]
pub struct ClassificationExpectation(ic::IrisSpecies);

impl IntExpect for ClassificationExpectation {
    type ProvidedInt = u8;
    type BoolExpectation = crate::util::BoolExpectation;
    fn is_met_by(&self, value: Self::ProvidedInt) -> bool {
        use ic::IrisSpecies as S;
        match self.0 {
            S::Setosa => [0b_10, 0b_11].contains(&value),
            S::Versicolor => 0b_00 == value,
            S::Virginica => 0b_01 == value,
        }
    }

    fn bit_expectation(&self, idx: usize) -> Self::BoolExpectation {
        use ic::IrisSpecies as S;
        match self.0 {
            S::Setosa => [BoolExpectation::NoExpect, BoolExpectation::Expect(true)][idx],
            S::Versicolor | S::Virginica => {
                [(self.0 == S::Virginica).expectation(), false.expectation()][idx]
            }
        }
    }
}
