use std::fs::write;

use rodio::{OutputStream, Source};

use crate::providers::local_story_provider::get_story_destination;
use crate::services::teller::Teller;
use crate::use_cases::play_story::{PlayStory, PlayStoryInput};

mod providers;
mod errors;
mod entities;
mod use_cases;
mod services;

fn main() {
    init();

    // loop here

    play(String::from("789"));
}

fn play(id: String) {
    let mut play_story = PlayStory::new();
    let mut story_filename: Option<String>;

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
            for device in Teller::list_host_devices() {
                let dev: rodio::Device = device.into();
                let source = Teller::get_source(&s).unwrap();
                let (_stream, mut stream_handle) = OutputStream::try_from_device(&dev).unwrap();
                let success = match stream_handle.play_raw(source.convert_samples()) {
                    Ok(_) => true,
                    Err(e) => {
                        println!("{:?}", e);
                        false
                    }
                };

                if success {
                    loop {}
                }
            }
        }
    }

    loop {}
}

fn init() {
    let test_mp3 = include_bytes!("resources/1-second-of-silence.mp3");
    let test_destination = match get_story_destination(String::from("test")) {
        Ok(s) => s,
        Err(err) => {
            Teller::tell_error(err);

            return;
        }
    };
    write(test_destination, test_mp3).expect("Could not write test mp3");
}
