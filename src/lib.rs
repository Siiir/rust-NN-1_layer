use core::panic;

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

pub fn create_classifier(
    mut classified_irises: Vec<ic::ClassifiedIris>,
) -> impl Fn(ic::UnclassifiedIris) -> ic::ClassifiedIris {
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
    .unwrap();

    move |unclassified_iris| {
        let prediction = nn.decide_for(unclassified_iris.as_na_svec());
        let classification = prediction_to_classification(prediction);
        ic::ClassifiedIris::new(unclassified_iris, classification)
    }
}

fn prediction_to_classification(prediction: u8) -> ic::IrisSpecies {
    use ic::IrisSpecies as S;
    match prediction {
        0b_00 => S::Versicolor,
        0b_01 => S::Virginica,
        0b_10 | 0b_11 => S::Setosa,
        _ => panic!("Logic error: function didn't expect `prediction` > 3 ."),
    }
}
