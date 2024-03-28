use core::panic;

use anyhow::Context;
pub use nn::one_layer::OneLayerNN;
pub mod nn;

pub use perceptron::Perceptron;
pub mod perceptron;

#[allow(deprecated)]
pub use app::{
    args::AppArgs,
    cfg::{app_cfg, AppCfg, APP_CFG},
};
#[deprecated]
pub mod app;

pub mod util;

use class_expectation::ClassificationExpectation;
mod class_expectation;

/// Creates an iris classifier that is based on 1-layer neural network.
///
/// This function is non-deterministic. Meaning it can return different classifiers on each run.
/// Optimizations are suboptimal. If you are lucky, a random shuffle will order training data optimaly making the training more effective. Thus, returning a classifier with high accuracy. This randomness is expected to have small impact on huge data sets.
pub fn create_classifier(
    mut classified_irises: Vec<ic::ClassifiedIris>,
) -> anyhow::Result<impl Fn(ic::UnclassifiedIris) -> ic::ClassifiedIris> {
    use rand::prelude::*;
    classified_irises.shuffle(&mut thread_rng());

    let mut nn = crate::OneLayerNN::<2, 4>::default();
    nn.fit_to::<_, _, _, ClassificationExpectation>(
        classified_irises
            .iter()
            .map(|ci| ci.parameters.as_na_svec()),
        classified_irises
            .iter()
            .map(|ci| ci.classification)
            .map(ClassificationExpectation::from),
        10,
    )
    .context("Provided training data is an empty table.")?;

    Ok(
        move |unclassified_iris: ic::UnclassifiedIris| -> ic::ClassifiedIris {
            let prediction = nn.decide_for(unclassified_iris.as_na_svec());
            let classification = prediction_to_classification(prediction);
            ic::ClassifiedIris::new(unclassified_iris, classification)
        },
    )
}

/// Converts neural network's prediction into a valid iris classification.
fn prediction_to_classification(prediction: u8) -> ic::IrisSpecies {
    use ic::IrisSpecies as S;
    match prediction {
        0b_00 => S::Versicolor,
        0b_01 => S::Virginica,
        0b_10 | 0b_11 => S::Setosa,
        _ => panic!("Logic error: function didn't expect `prediction` > 3 ."),
    }
}
