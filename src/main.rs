//! Raspberry Pi 4 demo.
//! This example makes use the `std` feature
//! and `anyhow` dependency to make error handling more ergonomic.
//!
//! # Connections
//!
//! - 3V3    = VCC
//! - GND    = GND
//! - GPIO9  = MISO
//! - GPIO10 = MOSI
//! - GPIO11 = SCLK (SCK)
//! - GPIO22 = NSS  (SDA)

use std::fs::write;

use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal_02 as embedded_hal;
use hal::{Delay, Pin, Spidev};
use hal::spidev::{SpidevOptions, SpiModeFlags};
use hal::sysfs_gpio::Direction;
use linux_embedded_hal as hal;
use mfrc522::comm::eh02::spi::SpiInterface;
use mfrc522::Mfrc522;
use rodio::{OutputStream, Sink};

use crate::providers::local_story_provider::get_story_destination;
use crate::services::teller::Teller;
use crate::use_cases::play_story::{PlayStory, PlayStoryInput};

mod entities;
mod errors;
mod providers;
mod services;
mod use_cases;

const PAUSE_TAG_ID: &str = "83.30.60.13";

fn main() -> Result<()> {
    init();

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    play(String::from("init"), &sink)
}

fn play(id: String, sink: &Sink) -> Result<()> {
    let mut delay = Delay;
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;

    // software-controlled chip select pin
    let pin = Pin::new(22);
    pin.export()?;
    while !pin.is_exported() {}
    delay.delay_ms(1u32); // delay sometimes necessary because `is_exported()` returns too early?
    pin.set_direction(Direction::Out)?;
    pin.set_value(1)?;

    // The `with_nss` method provides a GPIO pin to the driver for software controlled chip select.
    let itf = SpiInterface::new(spi).with_nss(pin);
    let mut mfrc522 = Mfrc522::new(itf).init()?;

    let vers = mfrc522.version()?;
    assert!(vers == 0x91 || vers == 0x92);

    let mut play_story = PlayStory::new();
    let story_filename: Option<String>;

    story_filename = match play_story.execute(PlayStoryInput { id }) {
        Ok(h) => Some(h),
        Err(error) => {
            Teller::tell_error(error);
            None
        }
    };

    match story_filename {
        None => (),
        Some(s) => {
            let source = Teller::get_source(&s).unwrap();
            sink.append(source);
            sink.play();
            delay.delay_ms(1000u32);
            loop {
                if let Ok(atqa) = mfrc522.reqa() {
                    if let Ok(uid) = mfrc522.select(&atqa) {
                        delay.delay_ms(100u32);
                        let id = uid
                            .as_bytes()
                            .to_vec()
                            .into_iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<String>>()
                            .join(".");
                        println!("Matched tag {}", &id);

                        if id.eq(PAUSE_TAG_ID) {
                            if sink.is_paused() {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                            delay.delay_ms(100u32);
                        } else {
                            sink.stop();
                            sink.clear();
                            return play(String::from(id), sink);
                        }
                    }
                }

                delay.delay_ms(500u32);
            }
        }
    };

    Ok(())
}

fn init() {
    let test_mp3 = include_bytes!("resources/init.mp3");
    let test_destination = match get_story_destination(String::from("init")) {
        Ok(s) => s,
        Err(err) => {
            Teller::tell_error(err);

            return;
        }
    };
    write(test_destination, test_mp3).expect("Could not write init mp3");
}
