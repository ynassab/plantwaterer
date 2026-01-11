#[cfg(feature = "gpio-real")]
use rppal;

pub trait OutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

#[cfg(feature = "gpio-real")]
pub mod gpio_rpi {
    use super::rppal::gpio::{Gpio, OutputPin as RpiOutputPin};
    use super::OutputPin;

    pub struct RpiPin {
        pin: RpiOutputPin,
    }
    
    #[allow(dead_code)]
    impl RpiPin {
        pub fn new(pin_number: u8) -> Self {
            let gpio = Gpio::new().expect("GPIO init failed.");
            let pin = gpio.get(pin_number).expect("Invalid pin").into_output();
            Self{ pin }
        }
    }

    impl OutputPin for RpiPin {
        fn set_high(&mut self) {
            self.pin.set_high();  // 3.3 V
        }

        fn set_low(&mut self) {
            self.pin.set_low();  // 0 V
        }
    }
}

#[cfg(feature = "gpio-mock")]
pub mod gpio_mock {
    use super::OutputPin;

    pub struct MockPin {
        pin: u8,
    }

    #[allow(dead_code)]
    impl MockPin {
        pub fn new(pin_number: u8) -> Self {
            println!("[MOCK GPIO] Initialising pin {}", pin_number);
            Self { pin: pin_number }
        }
    }

    impl OutputPin for MockPin {
        fn set_high(&mut self) {
            println!("[MOCK GPIO] Pin {} set to HIGH", self.pin);
        }

        fn set_low(&mut self) {
            println!("[MOCK GPIO] Pin {} set to LOW", self.pin);
        }
    }
}

