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
    
    if state.can_start_mixing() {
        log::info!("Starting mixing");
        save_state(&CycleState::MixingStarted)?;
        
        if let Err(e) = actuators.mixing_pump.run_for(mixing_time) {
            log::error!("Mixing step failed: {}", e);
            return Err(e.into());
        }
        
        save_state(&CycleState::MixingCompleted)?;
        log::info!("Completed mixing");
        state = CycleState::MixingCompleted;
    }

    if state.can_start_watering() {
        log::info!("Started watering");
        save_state(&CycleState::WateringStarted)?;
        
        if let Err(e) = actuators.water_pump.run_for(watering_time) {
            log::error!("Watering step failed: {}", e);
            return Err(e.into());
        }
        
        log::info!("Completed watering");
        save_state(&CycleState::Completed)?;
    }

    Ok(())
}
