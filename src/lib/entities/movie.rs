pub struct Movie {
    id: u64,
    title: String,
}

impl Movie {
    pub fn new(id: u64, title: String) -> Self {
        Self { id, title }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}