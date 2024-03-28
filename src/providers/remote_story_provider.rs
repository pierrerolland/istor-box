use crate::entities::story::Story;
use crate::errors::Error;
use crate::providers::local_story_provider::get_story_destination;
use crate::providers::remote_story_provider::implementation::fetch_story_contents_from_api;

mod implementation;

pub fn fetch_story(id: String) -> Result<Story, Error> {
    let destination = get_story_destination(id.clone())?;
    fetch_story_contents_from_api(id.clone(), destination.clone())?;

    Ok(Story::new(destination))
}
