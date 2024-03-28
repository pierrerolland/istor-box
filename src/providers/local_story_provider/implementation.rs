use std::env;
use std::fs::{create_dir_all, metadata, read_dir};

use crate::errors::Error;

pub fn check_stories_directory_existence() -> bool {
    let directory = get_stories_directory();

    match read_dir(&directory) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn create_stories_directory() -> Result<(), Error> {
    let directory = get_stories_directory();

    match create_dir_all(directory) {
        Ok(()) => Ok(()),
        Err(_) => Err(Error::CouldNotCreateStoriesDirectory),
    }
}

pub fn get_story_destination_filename(id: String) -> String {
    format!("{}/{}.mp3", get_stories_directory(), id)
}

pub fn read_story_file(id: String) -> Result<String, Error> {
    let full_path = format!("{}/{}.mp3", get_stories_directory(), id);

    match metadata(&full_path) {
        Ok(_) => Ok(full_path.clone()),
        Err(_) => Err(Error::CouldNotOpenLocalStoryFile),
    }
}

pub fn get_stories_directory() -> String {
    env::var("STORIES_DIRECTORY")
        .expect("Stories directory has to be defined in STORIES_DIRECTORY env var")
}
