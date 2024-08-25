use std::ascii::AsciiExt;

pub struct DocDetails {
    id: u64,
    title: String,
}

impl DocDetails {
    pub fn new(id: u64, mut title: String) -> Self {
        title.make_ascii_lowercase();
        Self {
            id
            ,
            title,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}