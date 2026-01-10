use super::config;

use crate::state::{load_state, save_state, CycleState};
use crate::errors::StdError;
use crate::hardware::actuators::Actuators;
use crate::hardware::relaypump::RelayPump;

use std::time::Duration;

pub fn build_actuators() -> Actuators {
    Actuators {
        mixing_pump: Box::new(RelayPump::new(config::MIXING_PUMP_PIN)),
        water_pump: Box::new(RelayPump::new(config::WATER_PUMP_PIN)),
    }
}

pub fn run_cycle(
    actuators: &mut Actuators,
    mixing_time: Duration,
    watering_time: Duration,
) -> Result<(), StdError> {

    let mut state = load_state();

    actuators.stop_all();  // Always first

    if matches!(state, CycleState::Idle | CycleState::MixingStarted) {
        save_state(&CycleState::MixingStarted)?;
        actuators.mixing_pump.run_for(mixing_time)?;
        save_state(&CycleState::MixingCompleted)?;
        state = CycleState::MixingCompleted;
    }

    if matches!(state, CycleState::MixingCompleted | CycleState::WateringStarted) {
        save_state(&CycleState::WateringStarted)?;
        actuators.water_pump.run_for(watering_time)?;
        save_state(&CycleState::Completed)?;
    }

    Ok(())
}
