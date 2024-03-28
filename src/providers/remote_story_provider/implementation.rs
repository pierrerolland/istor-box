use std::env;
use std::fs::write;

use crate::errors::Error;

pub fn fetch_story_contents_from_api(id: String, destination: String) -> Result<(), Error> {
    let url = format!("{}/api/story/{}", get_api(), id);

    let response = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(_) => {
            return Err(Error::CouldNotFetchRemoteStory);
        }
    };

    match response.bytes() {
        Ok(bytes) => match write(destination, bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::CouldNotCreateLocalStoryFile),
        },
        Err(_) => Err(Error::CouldNotDecodeRemoteStory),
    }
}

fn get_api() -> String {
    env::var("API_URL").expect("Remote story API has to be defined in API_URL env var")
}
