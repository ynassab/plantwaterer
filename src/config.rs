// BCM pin numbering
pub const MIXING_PUMP_PIN: u8 = 23;  // GPIO 23 = physical pin 16
pub const WATER_PUMP_PIN: u8 = 12;  // GPIO 12 = physical pin 32

pub const MIXING_TIME_SECS: u64 = 30;
pub const WATERING_TIME_SECS: u64 = 12;

pub const STATE_DIR_PATH: &str = ".";
pub const STATE_FILE_NAME: &str = "cycle_state.json";

pub const SIGNAL_HANDLER_CHECK_INTERVAL_SECONDS: u64 = 1;
