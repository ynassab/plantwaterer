use std::time::Duration;
use crate::signals::ShutdownFlag;
use crate::errors::StdError;
use crate::config::SIGNAL_HANDLER_CHECK_INTERVAL_SECONDS as INTERVAL;

pub trait Pump {
    fn start(&mut self) -> Result<(), StdError>;

    fn stop(&mut self) -> Result<(), StdError>;

    fn run_for(&mut self, duration: Duration) -> Result<(), StdError> {
        self.start()?;

        let shutdown = ShutdownFlag::new();

        for _ in 0..duration.as_secs() {
            if shutdown.is_requested() {
                log::warn!("Shutdown requested; stopping pumps");
                self.stop()?;
                return Err(StdError::from("Shutdown requested"));
            }
            std::thread::sleep(Duration::from_secs(INTERVAL));
        }

        self.stop()
    }
}
