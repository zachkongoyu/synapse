mod impulse;
use impulse::Impulse;

pub(crate) struct Protoplasm;

impl Protoplasm {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn flow(&self) -> Impulse {
        todo!()
    }
}
