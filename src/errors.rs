#[derive(Debug)]
pub enum Error {
    CouldNotCreateStoriesDirectory,
    CouldNotOpenLocalStoryFile,
    CouldNotCreateLocalStoryFile,
    CouldNotFetchRemoteStory,
    CouldNotDecodeRemoteStory,
    CouldNotPlayStory,
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        let str = match value {
            Error::CouldNotCreateStoriesDirectory => "Could not create stories directory",
            Error::CouldNotOpenLocalStoryFile => "Story not found locally",
            Error::CouldNotCreateLocalStoryFile => {
                "Found the story, but it could not be written locally"
            }
            Error::CouldNotFetchRemoteStory => "Didn't find the story remotely",
            Error::CouldNotDecodeRemoteStory => "The story isn't well formatted",
            Error::CouldNotPlayStory => "The story isn't playable",
        };

        String::from(str)
    }
}
