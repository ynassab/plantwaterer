use crate::errors::StdError;

pub struct Relay {
    pin_number: u8
}

impl Relay {
    pub fn new(pin_number: u8) -> Self {
        Self { pin_number }
    }
}

impl Relay {
    pub fn set_on(&self) -> Result<(), StdError> {
        // Placeholder implementation
        println!("Relay on pin {} set to ON", self.pin_number);
        Ok(())
    }

    pub fn set_off(&self) -> Result<(), StdError> {
        // Placeholder implementation
        println!("Relay on pin {} set to OFF", self.pin_number);
        Ok(())
    }
}
