use ic::{read, ClassifiedIris};
use perc_ic::{app_cfg, AppCfg, APP_CFG};

fn main() -> anyhow::Result<()> {
    let app_args: perc_ic::AppArgs = clap::Parser::parse();
    APP_CFG
        .set(AppCfg::new(app_args))
        .expect("This should be the only app config initialization.");

    // Reading iris data.
    let training_irises = read::training_irises()?;
    // Creating classifier using the classified data.
    let iris_classifier = perc_ic::create_classifier(training_irises);
    if app_cfg().run_accuracy_measure {
        ic::app::run_accuracy_measure(&iris_classifier)?;
    }
    let user_irises = read::user_irises()?;

    // Classifying all unclassified irises using classifier.
    let now_classified_irises: Vec<ClassifiedIris> =
        ic::classify_irises(&iris_classifier, user_irises);
    // Displaying the classifications made for user.
    let table_with_classified = tabled::Table::new(now_classified_irises);
    print!("{}", table_with_classified);

    Ok(())
}
