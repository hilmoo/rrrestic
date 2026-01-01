mod config;
mod runner;
mod telemetry;

use anyhow::Result;
use tracing::{error, info};

fn main() -> Result<()> {
    telemetry::init();

    let config_path =
        std::env::var("RRRESTIC_CONFIG").unwrap_or_else(|_| "rrrestic.toml".to_string());

    info!(path = config_path, "Loading configuration");

    let config = match config::Config::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            error!(error = ?e, "Fatal: Could not load config");
            std::process::exit(1);
        }
    };

    for job in config.backup {
        runner::run_job(&job);
    }

    Ok(())
}
