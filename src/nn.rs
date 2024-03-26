pub mod one_layer {
    use std::{num::NonZeroU64, ops::SubAssign};

    use crate::{
        perceptron::PerVec,
        util::{Correctness, IntExpect},
        Perceptron,
    };
    use num_rational::Ratio;
    use num_traits::{zero, One, PrimInt, Zero};
    use rayon::iter::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    };

    pub trait OutputInt: PrimInt + Send + Sync {}
    impl<T> OutputInt for T where T: PrimInt + Send + Sync {}

    #[derive(Debug)]
    pub struct OneLayerNN<const N: usize, const D: usize> {
        perceptrons: [Perceptron<D>; N],
    }

    /// Implements `Default` for provided values of `N`.
    macro_rules! impl_default {
        ($N: expr) => {
            impl<const D: usize> Default for OneLayerNN<$N, D> {
                fn default() -> Self {
                    Self {
                        perceptrons: Default::default(),
                    }
                }
            }
        };
        ($($N: expr),*) => {
            $(impl_default!($N);)*
        }
    }
    impl_default!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

    impl<const N: usize, const D: usize> OneLayerNN<N, D> {
        // CRUD-R: Properties

        pub fn decide_for<I>(&self, input: &PerVec<D>) -> I
        where
            I: OutputInt,
        {
            self.perceptrons
                .par_iter()
                .enumerate()
                .map(|(idx, perceptron)| (idx, perceptron.decide_for(input)))
                .fold(I::zero, |acc, (index, decision)| {
                    let bit = if decision { I::one() } else { I::zero() };
                    acc | (bit << index)
                })
                .reduce(I::zero, |a, b| a | b)
        }
        /// `E as PartialEq<I>` must be equivalence relation.
        pub fn accuracy_for<
            'i,
            II: IntoIterator<Item = &'i PerVec<D>>,
            EI: IntoIterator<Item = E>,
            I: OutputInt,
            E: IntExpect<ProvidedInt = I>,
        >(
            &self,
            inputs: II,
            expected: EI,
        ) -> Option<Ratio<u64>> {
            let [mut correct, mut all] = [0, 0];
            for (input, expectations) in inputs.into_iter().zip(expected) {
                if expectations.is_met_by(self.decide_for::<I>(input)) {
                    correct += 1;
                }
                all += 1;
            }
            Some(Ratio::new(correct, NonZeroU64::new(all)?.get()))
        }

        // CRUD-U: Training [`self`].

        /// `E as PartialEq<I>` must be equivalence relation.
        pub fn train_on_sample<I, E>(
            &mut self,
            input: &PerVec<D>,
            expectation: E,
        ) -> crate::util::Correctness
        where
            I: OutputInt,
            E: IntExpect + Sync + Send,
        {
            if self
                .perceptrons
                .par_iter_mut()
                .enumerate()
                .map(move |(idx, perceptron)| {
                    perceptron.train_on_sample(input, expectation.bit_expectation(idx))
                })
                .all(|correctness| correctness.is_correct())
            {
                Correctness::Correct
            } else {
                Correctness::Incorrect
            }
        }

        /// `E as PartialEq<I>` must be equivalence relation.
        pub fn train_on<'i, II, EI, I, E>(
            &mut self,
            inputs: II,
            expecteds: EI,
        ) -> Option<Ratio<u64>>
        where
            II: IntoIterator<Item = &'i PerVec<D>>,
            EI: IntoIterator<Item = E>,
            I: OutputInt,
            E: IntExpect<ProvidedInt = I> + Sync + Send,
        {
            let [mut correct, mut all] = [0, 0];
            for (input, expected) in inputs.into_iter().zip(expecteds) {
                if self.train_on_sample::<I, E>(input, expected).is_correct() {
                    correct += 1;
                }
                all += 1;
            }
            Some(Ratio::new(correct, NonZeroU64::new(all)?.get()))
        }

        /// If you don't know the [`old_score`]:
        /// + Pass `None` to [`old_score`] if you expect this algorithm to iterate exactly once.
        /// + Pass zero to [`old_score`] otherwise.
        pub fn fit<'i, II, EI, I, E, C>(
            &mut self,
            inputs: II,
            expecteds: EI,
            max_progress_reattemps: u64,
            max_iterations: C,
            mut old_score: Option<Ratio<u64>>,
        ) -> Option<Ratio<u64>>
        where
            II: Clone + IntoIterator<Item = &'i PerVec<D>>,
            EI: Clone + IntoIterator<Item = E>,
            I: OutputInt,
            E: IntExpect<ProvidedInt = I> + Sync + Send,
            C: Zero + One + SubAssign,
        {
            if old_score.is_none() {
                old_score = self.accuracy_for(inputs.clone(), expecteds.clone())
            }
            let mut new_score = None;
            let mut reattemps_left = max_progress_reattemps;

            let mut iter_to_perform = max_iterations;
            while !iter_to_perform.is_zero() {
                new_score = self.train_on::<II, EI, I, E>(inputs.clone(), expecteds.clone());
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
                iter_to_perform -= C::one();
            }
            new_score
        }

        pub fn fit_to<'i, II, EI, I, E>(
            &mut self,
            inputs: II,
            expecteds: EI,
            max_progress_reattemps: u64,
        ) -> Option<Ratio<u64>>
        where
            II: Clone + IntoIterator<Item = &'i PerVec<D>>,
            EI: Clone + IntoIterator<Item = E>,
            I: OutputInt,
            E: IntExpect<ProvidedInt = I> + Sync + Send,
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
}
