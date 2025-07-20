use embedded_hal::digital::v2::OutputPin;
use embedded_hal_02 as embedded_hal;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

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

impl OutputPin for CdevOutputPin {
    type Error = gpio_cdev::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.handle.set_value(0)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.handle.set_value(1)
    }
}
