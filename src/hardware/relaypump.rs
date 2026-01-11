use super::pump::Pump;
use super::relay::Relay;
use crate::errors::StdError;

pub struct RelayPump {
    relay: Relay,
}

impl RelayPump {
    pub fn new(pin_number: u8) -> Self {
        Self {
            relay: Relay::new(pin_number).unwrap(),
        }
    }
}

impl Pump for RelayPump {
    fn start(&mut self) -> Result<(), StdError> {
        self.relay.set_on()
    }

    fn stop(&mut self) -> Result<(), StdError> {
        self.relay.set_off()
    }
}
