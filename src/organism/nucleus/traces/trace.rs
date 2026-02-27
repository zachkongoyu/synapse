pub struct Trace {
    id: u64,
    content: String,
}

impl Trace {
    pub(super) fn new(id: u64, content: String) -> Self {
        Self { id, content }
    }
}
