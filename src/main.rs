mod config;
mod controller;
mod errors;
mod hardware;
mod signals;
mod state;

use crate::controller::build_actuators;
use crate::signals::ShutdownFlag;
use crate::errors::StdError;

use std::time::Duration;

fn main() -> Result<(), StdError> {
    // env_logger::init();

    let shutdown = ShutdownFlag::new();
    signals::install_signal_handler(shutdown.clone());

    let mut actuators = build_actuators();
    actuators.stop_all();  // Startup safety

    {
        controller::run_cycle(
            &mut actuators,
            Duration::from_secs(config::MIXING_TIME_SECS),
            Duration::from_secs(config::WATERING_TIME_SECS),
        )?;
    }

    Ok(())
}
