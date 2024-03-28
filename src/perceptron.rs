use std::{num::NonZeroU64, ops::SubAssign};

pub use nalgebra as na;
use num_rational::Ratio;
use num_traits::{one, zero, One, Zero};

use crate::util::{BoolExpect, BoolExpectation, Correctness};

pub type PerFloat = f32;
pub type PerVec<const D: usize> = na::SVector<PerFloat, D>;

#[derive(Clone, Debug)]
pub struct Perceptron<const D: usize> {
    wages: PerVec<D>,
    theta: PerFloat,
}

impl<const D: usize> Default for Perceptron<D> {
    fn default() -> Self {
        Self {
            wages: na::SVector::zeros(),
            theta: 0.,
        }
    }
}
impl<const D: usize> Perceptron<D> {
    pub const ALPHA: PerFloat = 0.1;

    // CRUD-R: Properties

    /// Activation function.
    pub fn activation(&self, value: PerFloat) -> bool {
        value >= self.theta
    }
    /// Returns decision for given [`input`].
    pub fn decide_for(&self, input: &PerVec<D>) -> bool {
        let dot_prod = crate::util::sf32_vec::dot(&self.wages, input);
        self.activation(dot_prod)
    }
    /// Returns accuracy this classifier has for the given test data.
    pub fn accuracy_for<'i, II, EI, E>(&self, inputs: II, expectations: EI) -> Option<Ratio<u64>>
    where
        II: IntoIterator<Item = &'i PerVec<D>>,
        EI: IntoIterator<Item = E>,
        E: crate::util::BoolExpect,
    {
        let [mut correct, mut all] = [0, 0];
        for (input, expectation) in inputs.into_iter().zip(expectations) {
            if expectation.is_met_by(self.decide_for(input)) {
                correct += 1;
            }
            all += 1;
        }
        Some(Ratio::new(correct, NonZeroU64::new(all)?.get()))
    }

    // CRUD-U: Training [`self`].

    pub fn train_on_sample<E>(&mut self, input: &PerVec<D>, expectation: E) -> Correctness
    where
        E: BoolExpect,
    {
        let prediction = self.decide_for(input);
        match expectation.expectation() {
            BoolExpectation::Expect(expectation) => {
                if expectation.is_met_by(prediction) {
                    return Correctness::Correct; // Correct, no need to improve
                }
                let translation_dir = match [expectation, prediction] {
                    [false, true] => -1.,
                    [true, false] => 1.,
                    _ => {
                        // Do nothing, nothing to improve
                        return Correctness::Correct;
                    }
                };
                let translation_multiplier = translation_dir * Self::ALPHA;
                // Update self
                crate::util::sf32_vec::add_assign(
                    &mut self.wages,
                    &(translation_multiplier * input),
                );
                self.theta -= translation_multiplier; // Input is -1.
                Correctness::Incorrect // BUT improved
            }
            BoolExpectation::NoExpect => {
                // No expectation ==> nothing to do ==> everything is ok
                Correctness::Correct
            }
        }
    }

    pub fn train_on<'i, II, EI, E>(&mut self, inputs: II, expecteds: EI) -> Option<Ratio<u64>>
    where
        II: IntoIterator<Item = &'i PerVec<D>>,
        EI: IntoIterator<Item = E>,
        E: BoolExpect,
    {
        let [mut correct, mut all] = [0, 0];
        for (input, expected) in inputs.into_iter().zip(expecteds) {
            if self.train_on_sample(input, expected) == Correctness::Correct {
                correct += 1;
            }
            all += 1;
        }
        Some(Ratio::new(correct, NonZeroU64::new(all)?.get()))
    }

    /// If you don't know the [`old_score`]:
    /// + Pass `None` to [`old_score`] if you expect this algorithm to iterate exactly once.
    /// + Pass zero to [`old_score`] otherwise.
    pub fn fit<'i, II, EI, E, C>(
        &mut self,
        inputs: II,
        expecteds: EI,
        max_progress_reattemps: u64,
        max_iterations: C,
        mut old_score: Option<Ratio<u64>>,
    ) -> Option<Ratio<u64>>
    where
        II: Copy + IntoIterator<Item = &'i PerVec<D>>,
        EI: Copy + IntoIterator<Item = E>,
        E: BoolExpect,
        C: Zero + One + SubAssign,
    {
        if old_score.is_none() {
            old_score = self.accuracy_for(inputs, expecteds)
        }
        let mut new_score = None;
        let mut reattemps_left = max_progress_reattemps;

        let mut iter_to_perform = max_iterations;
        while !iter_to_perform.is_zero() {
            new_score = self.train_on(inputs, expecteds);
            use std::cmp::Ordering as O;
            match new_score.cmp(&old_score) {
                O::Less | O::Equal => {
                    // No-progress
                    if reattemps_left == 0 {
                        return new_score;
                    }
                    reattemps_left -= 1;
                }
                O::Greater => {
                    // Progress ==> It's cool. ==> Let's continue.
                    old_score = new_score;
                    // Give algorithm more chances (reset them).
                    reattemps_left = max_progress_reattemps;
                }
            }
            iter_to_perform -= one();
        }
        new_score
    }
    pub fn fit_to<'i, II, EI, E, C>(
        &mut self,
        inputs: II,
        expecteds: EI,
        max_progress_reattemps: u64,
    ) -> Option<Ratio<u64>>
    where
        II: Copy + IntoIterator<Item = &'i PerVec<D>>,
        EI: Copy + IntoIterator<Item = E>,
        E: BoolExpect,
    {
        return self.fit(
            inputs,
            expecteds,
            max_progress_reattemps,
            crate::util::OneAbsorbingSubtractions,
            Some(zero()),
        );
    }
}
