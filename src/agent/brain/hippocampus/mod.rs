mod memory;
use memory::Memory;

pub(crate) struct Hippocampus {
    memories: Vec<Memory>,
}
 
impl Hippocampus {
    pub (crate) fn new() -> Self {
        Self {
            memories: Vec::new(),
        }
    }

    pub (crate) fn memorize() -> Memory {
        todo!()
    }

    pub (crate) fn store(&mut self, memory: Memory) {
        let _ = memory;
        todo!()
    }

    pub (crate) fn recall(&self) -> Vec<Memory> {
        todo!()
    }

    pub (crate) fn forget(&mut self, memory_id: usize) {
        let _ = memory_id;
        todo!()
    }
}