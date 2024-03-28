#[derive(Clone)]
pub struct Story {
    contents: String,
}

impl Story {
    pub fn new(contents: String) -> Self {
        Story { contents }
    }

    pub fn get_contents(&self) -> String {
        self.contents.clone()
    }
}
