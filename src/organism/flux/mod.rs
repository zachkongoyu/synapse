mod impulse;
use impulse::Impulse;

pub(crate) struct Flux;

impl Flux {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn flow(&self) -> Impulse {
        todo!()
    }
}
