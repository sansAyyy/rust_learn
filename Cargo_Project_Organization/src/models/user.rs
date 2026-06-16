#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> String {
        format!("#{} {}", self.id, self.name)
    }
}
