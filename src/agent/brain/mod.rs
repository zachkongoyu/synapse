mod cortex;
mod hippocampus;

use cortex::Cortex;
use hippocampus::Hippocampus;

pub(crate) struct Brain {
    pub (crate) cortex: Cortex,
    pub (crate) hippocampus: Hippocampus,
}

impl Brain {
    pub(crate) fn new() -> Self {
        Self {
            cortex: Cortex::new(),
            hippocampus: Hippocampus::new(),
        }
    }
}