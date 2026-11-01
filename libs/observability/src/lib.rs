use tracing_subscriber::EnvFilter;

use crate::{config::ObservabilityConfig, error::ObservabilityError};

pub mod config;
pub mod error;

pub fn init_observability(config: &ObservabilityConfig) -> anyhow::Result<(), ObservabilityError> {
    let mut filter = EnvFilter::new(&config.logging.level);

    if let Some(directives) = &config.logging.directives {
        for directive in directives {
            let parsed_directive = directive
                .parse()
                .map_err(ObservabilityError::DirectiveParse)?;
            filter = filter.add_directive(parsed_directive);
        }
    }

    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .with_env_filter(filter)
        .init();

    Ok(())
}
