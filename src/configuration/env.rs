use std::sync::OnceLock;
use config::File;
use crate::util::log_unwrap::LogUnwrap;
use super::Configuration;

static ENV_CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

impl Configuration {
    pub fn env() -> &'static Self {
        ENV_CONFIGURATION
            .get_or_init(|| {
                config::Config::builder()
                    .add_source(
                        File::with_name("config.yaml").format(config::FileFormat::Yaml)
                    )
                    .build()
                    .log_unwrap()
                    .try_deserialize()
                    .log_unwrap()
            })
    }
}