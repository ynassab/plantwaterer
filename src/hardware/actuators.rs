use super::pump::Pump;

pub struct Actuators {
    pub mixing_pump: Box<dyn Pump>,
    pub water_pump: Box<dyn Pump>,
}

impl Actuators {
    pub fn new(mixing_pump: Box<dyn Pump>, water_pump: Box<dyn Pump>) -> Self {
        Self {
            mixing_pump,
            water_pump,
        }
    }

    pub fn stop_all(&mut self) {
        let _ = self.mixing_pump.stop();
        let _ = self.water_pump.stop();
    }
}

impl Drop for Actuators {
    fn drop(&mut self) {
        self.stop_all();
    }
}
