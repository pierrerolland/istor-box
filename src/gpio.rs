use embedded_hal::digital::OutputPin;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use linux_embedded_hal::CdevPinError;

pub struct CdevOutputPin {
    handle: LineHandle,
}

impl CdevOutputPin {
    pub fn new(line: u32) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handle = chip
            .get_line(line)?
            .request(LineRequestFlags::OUTPUT, 1, "istor-box")?;
        Ok(Self { handle })
    }
}

impl embedded_hal::digital::ErrorType for CdevOutputPin {
    type Error = CdevPinError;
}

impl OutputPin for CdevOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.handle.set_value(0).map_err(|e| e.into())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.handle.set_value(1).map_err(|e| e.into())
    }
}
