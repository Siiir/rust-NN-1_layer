pub mod perceptron {
    pub use nalgebra as na;
    use num_rational::Ratio;

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

        pub fn activation(&self, value: Float) -> bool {
            value >= self.theta
        }
        pub fn decide_for(&self, input: &PerVec<D>) -> bool {
            let dot_prod = self.wages.transpose() * input;
            self.activation(dot_prod.x)
        }
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
        pub fn train_on<'i>(
            &mut self,
            inputs: impl IntoIterator<Item = &'i PerVec<D>>,
            expected: impl IntoIterator<Item = bool>,
        ) -> Option<Ratio<u64>> {
            let [mut correct, mut all] = [0, 0];
            for (input, expected) in inputs.into_iter().zip(expected) {
                if self.train_on_sample(input, expected) {
                    correct += 1;
                }
                all += 1;
            }
            Some(Ratio::new(correct, all))
        }
        pub fn fit<
            'i,
            II: Copy + IntoIterator<Item = &'i PerVec<D>>,
            EI: Copy + IntoIterator<Item = bool>,
        >(
            &mut self,
            inputs: II,
            expected: EI,
            max_iterations: u64,
            max_reattemps: u64,
        ) -> Option<Ratio<u64>> {
            let mut old_score = None;
            let mut new_score = None;
            let mut reattemps_left = max_reattemps;

            for _ in 0..=max_iterations {
                new_score = self.train_on(inputs, expected);
                use std::cmp::Ordering as O;
                match new_score.cmp(&old_score) {
                    o @ (O::Less | O::Equal) => {
                        // No-progress
                        if reattemps_left == 0 {
                            return new_score;
                        }
                        reattemps_left -= 1;
                    }
                    O::Greater => {
                        // Progress ==> It's cool. ==> Let's continue.
                        old_score = new_score;
                        // Give algorithm more chances.
                        reattemps_left = max_reattemps;
                    }
                }
            }
            new_score
        }
    }
}
