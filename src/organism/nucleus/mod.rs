mod protoplasm;
mod traces;

use protoplasm::Protoplasm;
use traces::Traces;

pub(crate) struct Nucleus {
    pub(crate) protoplasm: Protoplasm,
    pub(crate) traces: Traces,
}

impl Nucleus {
    pub(crate) fn new() -> Self {
        Self {
            protoplasm: Protoplasm::new(),
            traces: Traces::new(),
        }
    }
}
