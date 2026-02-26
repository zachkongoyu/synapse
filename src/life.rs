use crate::agent::brain::cortex::{Cortex, Thought};
use crate::agent::brain::memory::Memory;

pub(crate) struct ExternalState {}

pub(crate) struct InternalState {
    memory: Memory,
}

pub(crate) struct Life {
    pub internal_state: InternalState,
    pub external_state: ExternalState,
}

impl Life {
    pub(crate) async fn living(&mut self, cortex: &Cortex) -> ! {
        let _ = cortex;

        loop {
            let thought = self.think(cortex).await;
            self.interact(&thought).await;
            self.update_state(&thought).await;
        }
    }

    async fn think(&self, cortex: &Cortex) -> Thought {
        let _ = (self, cortex);
        todo!()
    }

    async fn interact(&self, thought: &Thought) {
        let _ = thought;
        todo!()
    }

    async fn update_state(&mut self, thought: &Thought) {
        let _ = thought;
        todo!()
    }
}
