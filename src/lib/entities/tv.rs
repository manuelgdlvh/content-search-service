pub struct Tv {
    id: u64,
    title: String,
}

impl Tv {
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