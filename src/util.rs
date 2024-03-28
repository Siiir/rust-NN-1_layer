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

/// Student's implementations of operations on `nalgebra` vectors.
///
/// These are demanded by teacher as a part of this project.
/// They are hidden behind optional feature `student_impls`.
/// Therefore the app will use `nalgebra` impls in default compilation.
/// This setup is intended to prevent performance drawbacks if app is run by normal user.
/// Yet it is possible for teacher to compile app using student implementations and test them.
pub mod sf32_vec {
    use crate::perceptron::{PerFloat, PerVec};

    pub fn dot<const D: usize>(lhs: &PerVec<D>, rhs: &PerVec<D>) -> PerFloat {
        cfg_if::cfg_if! {
            if #[cfg(feature = "student_impls")]{
                lhs.iter()
                    .zip(rhs.iter())
                    .map(|(l_el, r_el)| l_el * r_el)
                    .sum()
            }else{
                (lhs.transpose() * rhs).x
            }
        }
    }
    pub fn add_assign<const D: usize>(lhs: &mut PerVec<D>, rhs: &PerVec<D>) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "student_impls")]{
                lhs.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(l_el, r_el)| *l_el += r_el)
            }else{
                *lhs += rhs
            }
        }
    }
}
