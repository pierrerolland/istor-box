use crate::entities::story::Story;
use crate::errors::Error;
use crate::providers::local_story_provider::implementation::{check_stories_directory_existence, create_stories_directory, get_story_destination_filename, read_story_file};

mod implementation;

pub fn retrieve_story(id: String) -> Result<Story, Error> {
    if !check_stories_directory_existence() {
        create_stories_directory()?;
    }

    let contents = read_story_file(id.clone())?;

    Ok(Story::new(id, contents))
}

pub fn get_story_destination(id: String) -> Result<String, Error> {
    if !check_stories_directory_existence() {
        create_stories_directory()?;
    }

    Ok(get_story_destination_filename(id))
}
