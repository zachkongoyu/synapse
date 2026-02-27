pub struct Memory {
    id: u64,
    content: String,
}

impl Memory {
    pub(super) fn new(id: u64, content: String) -> Self {
        Self { id, content }
    }
}