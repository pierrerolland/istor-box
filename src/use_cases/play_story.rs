use crate::entities::story::Story;
use crate::errors::Error;
use crate::providers::local_story_provider::retrieve_story;
use crate::providers::remote_story_provider::fetch_story;
use crate::services::teller::Teller;

pub struct PlayStoryInput {
    pub id: String,
}

pub struct PlayStory {}

impl PlayStory {
    pub fn new() -> Self {
        PlayStory {}
    }

    pub fn execute(&mut self, input: PlayStoryInput) -> Result<String, Error> {
        let story = self.get_story(input.id)?;

        Ok(Teller::tell_the_story(&story))
    }

    fn get_story(&self, id: String) -> Result<Story, Error> {
        match retrieve_story(id.clone()) {
            Ok(story) => Ok(story),
            Err(error) => match error {
                Error::CouldNotOpenLocalStoryFile => self.fetch_remotely(id),
                _ => Err(error),
            },
        }
    }

    fn fetch_remotely(&self, id: String) -> Result<Story, Error> {
        Teller::tell_we_fetch_remotely();
        fetch_story(id.clone())?;

        retrieve_story(id)
    }
}
