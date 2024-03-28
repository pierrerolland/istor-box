use std::fs::File;
use std::io::BufReader;

use rodio::Decoder;

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

    pub fn get_source(filename: &String) -> Result<Decoder<BufReader<File>>, Error> {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => {
                return Err(Error::CouldNotOpenLocalStoryFile);
            }
        };
        let buffer = BufReader::new(file);
        let source = match Decoder::new(buffer) {
            Ok(src) => src,
            Err(_) => {
                return Err(Error::CouldNotDecodeRemoteStory);
            }
        };

        Ok(source)
    }
}
