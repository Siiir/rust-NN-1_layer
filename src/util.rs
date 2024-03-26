pub use one_absorbing_subtractions::OneAbsorbingSubtractions;
pub mod one_absorbing_subtractions;

pub use int_expect::IntExpect;
pub mod int_expect;

pub use bool_expect::BoolExpect;
pub mod bool_expect;

pub use bool_expectation::BoolExpectation;
pub mod bool_expectation;

pub use correctness::Correctness;
pub mod correctness {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Correctness {
        Correct,
        Incorrect,
    }
    impl Correctness {
        pub fn is_correct(&self) -> bool {
            *self == Self::Correct
        }
    }
}
