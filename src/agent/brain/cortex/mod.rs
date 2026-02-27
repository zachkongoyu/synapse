mod thought;
use thought::Thought;

pub(crate) struct Cortex;

impl Cortex {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn think(&self) -> Thought {
        todo!()
    }
}