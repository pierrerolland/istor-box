#[derive(Clone)]
pub struct Story {
    id: String,
    contents: String,
}

impl Story {
    pub fn new(id: String, contents: String) -> Self {
        Story {
            id,
            contents,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_contents(&self) -> String {
        self.contents.clone()
    }
}
