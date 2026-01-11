use crate::errors::StdError;
use super::platformpin::OutputPin;

#[cfg(feature = "gpio-real")]
use super::platformpin::gpio_rpi::RpiPin as PlatformPin;

#[cfg(feature = "gpio-mock")]
use super::platformpin::gpio_mock::MockPin as PlatformPin;

pub struct Relay {
    pin: PlatformPin,
}

impl Relay {
    pub fn new(pin_number: u8) -> Result<Self, StdError> {
        Ok(Self{ pin: PlatformPin::new(pin_number) })
    }
}

impl Relay {
    pub fn set_on(&mut self) -> Result<(), StdError> {
        self.pin.set_high();
        Ok(())
    }

    pub fn set_off(&mut self) -> Result<(), StdError> {
        self.pin.set_low();
        Ok(())
    }
}

