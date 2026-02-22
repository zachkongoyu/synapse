use serde_json::Value;
use crate::agent::brain::memory::Memory;

struct Cortex {
    mind: Box<dyn Provider>,
    identity: String,
    tool_schemas: Vec<Value>,
}

enum Thought {}

impl Cortex {
    fn think(&self, memory: &Memory) -> Thought {
        let messages = memory.read(None);

        let response = self.mind.process(messages);
    }
}