pub mod perceptron {
    use std::ops::{Add, Mul, SubAssign};

    pub use nalgebra as na;
    use num_rational::Ratio;
    use num_traits::{one, zero, One, Zero};

    pub type Float = f32;
    pub type PerVec<const D: usize> = na::SVector<Float, D>;

    #[derive(Clone, Debug)]
    pub struct Perceptron<const D: usize> {
        wages: PerVec<D>,
        theta: Float,
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
        pub const ALPHA: Float = 1.0;
        pub const BETA: Float = 1.0;

        // CRUD-R: Properties

        pub fn activation(&self, value: Float) -> bool {
            value >= self.theta
        }
        pub fn decide_for(&self, input: &PerVec<D>) -> bool {
            let dot_prod = self.wages.transpose() * input;
            self.activation(dot_prod.x)
        }
        pub fn accuracy_for<'i>(
            &self,
            inputs: impl IntoIterator<Item = &'i PerVec<D>>,
            expected: impl IntoIterator<Item = bool>,
        ) -> Option<Ratio<u64>> {
            let [mut correct, mut all] = [0, 0];
            for (input, expected) in inputs.into_iter().zip(expected) {
                if self.decide_for(input) == expected {
                    correct += 1;
                }
                all += 1;
            }
            Some(Ratio::new(correct, all))
        }

        // CRUD-U: Training [`self`].

        pub fn train_on_sample(&mut self, input: &PerVec<D>, expected: bool) -> bool {
            let decision = self.decide_for(input);
            if decision == expected {
                return true; // Correct, no need to improve
            }
            let [decision, expected] = [decision, expected].map(Float::from);
            // Update self
            self.wages += (decision - expected) * Self::ALPHA * input;
            self.theta -= (decision - expected) * Self::BETA; // Input is -1.
            false // incorrect, BUT improved
        }

        pub fn train_on<'i, II, EI>(&mut self, inputs: II, expecteds: EI) -> Option<Ratio<u64>>
        where
            II: IntoIterator<Item = &'i PerVec<D>>,
            EI: IntoIterator<Item = bool>,
        {
            let [mut correct, mut all] = [0, 0];
            for (input, expected) in inputs.into_iter().zip(expecteds) {
                if self.train_on_sample(input, expected) {
                    correct += 1;
                }
                all += 1;
            }
            Some(Ratio::new(correct, all))
        }

        /// If you don't know the [`old_score`]:
        /// + Pass `None` to [`old_score`] if you expect this algorithm to iterate exactly once.
        /// + Pass zero to [`old_score`] otherwise.
        pub fn fit<'i, II, EI, C>(
            &mut self,
            inputs: II,
            expecteds: EI,
            max_progress_reattemps: u64,
            max_iterations: C,
            mut old_score: Option<Ratio<u64>>,
        ) -> Option<Ratio<u64>>
        where
            II: Copy + IntoIterator<Item = &'i PerVec<D>>,
            EI: Copy + IntoIterator<Item = bool>,
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
        pub fn fit_to<'i, II, EI, C>(
            &mut self,
            inputs: II,
            expecteds: EI,
            max_progress_reattemps: u64,
        ) -> Option<Ratio<u64>>
        where
            II: Copy + IntoIterator<Item = &'i PerVec<D>>,
            EI: Copy + IntoIterator<Item = bool>,
        {
            return self.fit(
                inputs,
                expecteds,
                max_progress_reattemps,
                OneAbsorbingSubtractions,
                Some(zero()),
            );

            struct OneAbsorbingSubtractions;
            fn panic_pupet() -> ! {
                panic!("Logic error: Puppet method called.")
            }
            impl Zero for OneAbsorbingSubtractions {
                fn zero() -> Self {
                    panic_pupet()
                }

                fn is_zero(&self) -> bool {
                    false
                }
            }
            impl One for OneAbsorbingSubtractions {
                fn one() -> Self {
                    Self
                }
            }
            impl Add for OneAbsorbingSubtractions {
                type Output = Self;

                fn add(self, _: Self) -> Self::Output {
                    panic_pupet()
                }
            }
            impl Mul for OneAbsorbingSubtractions {
                type Output = Self;

                fn mul(self, _: Self) -> Self::Output {
                    panic_pupet()
                }
            }
            impl SubAssign for OneAbsorbingSubtractions {
                fn sub_assign(&mut self, _: Self) {
                    // Ignore subtraction
                }
            }
        }
    }
}
