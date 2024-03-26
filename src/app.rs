pub mod args {
    //! Facilitates usage of this app's arguments.

    use ic::util;

    #[derive(clap::Parser, Debug)]
    #[command(version, about, long_about = ic::executable_desc!())]
    pub struct AppArgs {
        /// Delimiter used for provided floating point values.
        ///
        /// Iris data should be provided in CSV format with separator being optionally overwriten by this option.
        #[arg(short, long, default_value_t = util::AsciiChar7Bit::COMMA)]
        pub separator: util::AsciiChar7Bit,

        /// Measures this classifier's accuracy using testing irises data.
        #[arg(short = 'a', long, default_value_t = true)]
        pub run_accuracy_measure: bool,
    }
}
pub mod cfg {
    //! Defines app's configuration.

    use core::panic;
    use std::sync::OnceLock;

    use derive_more::{Constructor, Deref, DerefMut};

    /// The only app configuration object.
    pub static APP_CFG: OnceLock<AppCfg> = OnceLock::new();

    /// Returns the global app configuration.
    ///
    /// # Panics
    /// * If it hasn't been initialized.
    pub fn app_cfg() -> &'static AppCfg {
        if let Some(app_cfg) = APP_CFG.get() {
            return app_cfg;
        } else {
            panic!("Logical error: app config used before being initialized.")
        }
    }

    /// App configuration.
    #[derive(Constructor, Debug, Deref, DerefMut)]
    pub struct AppCfg {
        app_args: crate::AppArgs,
    }
}
