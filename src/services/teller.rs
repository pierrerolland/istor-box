use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, Devices, OutputDevices, OutputStream, OutputStreamHandle};
use rodio::cpal::traits::{DeviceTrait, HostTrait};

use crate::entities::story::Story;
use crate::errors::Error;

pub struct Teller {}

impl Teller {
    pub fn tell_error(error: Error) {
        println!("{}", String::from(error));
    }

    pub fn tell_we_fetch_remotely() {
        println!("Could not find the story, trying to retrieve it remotely")
    }

    pub fn tell_the_story(story: &Story) -> String {
        story.get_contents()
    }

    fn tell(filename: String) -> Result<Decoder<BufReader<File>>, Error> {
        let source = Teller::get_source(&filename)?;

        Ok(source)
    }

    pub fn list_host_devices() -> OutputDevices<Devices> {
        let host = rodio::cpal::default_host();
        let devices = host.output_devices().unwrap();

        devices
        /*
        for device in devices {
            let dev: rodio::Device = device.into();
            let dev_name: String = dev.name().unwrap();
            println!(" # Device : {}", dev_name);
        }

         */
    }

    fn get_output_stream(device_name: &str) -> (OutputStream, OutputStreamHandle) {
        let host = rodio::cpal::default_host();
        let devices = host.output_devices().unwrap();
        let (mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();
        for device in devices {
            let dev: rodio::Device = device.into();
            let dev_name: String = dev.name().unwrap();
            if dev_name == device_name {
                println!("Device found: {}", dev_name);
                (_stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
            }
        }

        return (_stream, stream_handle);
    }

    pub fn get_source(filename: &String) -> Result<Decoder<BufReader<File>>, Error> {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => { return Err(Error::CouldNotOpenLocalStoryFile); }
        };
        let buffer = BufReader::new(file);
        let source = match Decoder::new(buffer) {
            Ok(src) => src,
            Err(_) => { return Err(Error::CouldNotDecodeRemoteStory); }
        };

        Ok(source)
    }
}
