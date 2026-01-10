use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use super::config::{STATE_DIR_PATH, STATE_FILE_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub enum CycleState {
    Idle,
    MixingStarted,
    MixingCompleted,
    WateringStarted,
    Completed,
}

impl CycleState {
    pub fn can_start_mixing(&self) -> bool {
        matches!(
            self,
            CycleState::Idle
                | CycleState::Completed
                | CycleState::MixingStarted
        )
    }

    pub fn can_start_watering(&self) -> bool {
        matches!(
            self,
            CycleState::MixingCompleted
                | CycleState::WateringStarted
        )
    }
}

pub fn state_file_path() -> PathBuf {
    Path::new(STATE_DIR_PATH).join(STATE_FILE_NAME)
}

pub fn load_state() -> CycleState {
    if let Ok(data) = fs::read_to_string(state_file_path()) {
        serde_json::from_str(&data).unwrap_or(CycleState::Idle)
    } else {
        CycleState::Idle
    }
}

pub fn save_state(state: &CycleState) -> std::io::Result<()> {
    fs::create_dir_all(STATE_DIR_PATH)?;
    fs::write(state_file_path(), serde_json::to_string(state)?)
}
