use serde_json::Value;
use crate::agent::brain::memory::Message;
use std::future::Future;
use std::pin::Pin;

mod thought;
pub(crate) use self::thought::Thought;

pub(crate) struct Cortex {
    mind: Box<dyn Provider>,
    identity: String,
    tool_schemas: Vec<Value>,
}

pub(crate) trait Provider: Send + Sync {
    fn process<'a>(
        &'a self,
        messages: &'a [crate::agent::brain::memory::Message],
    ) -> Pin<Box<dyn Future<Output = Value> + Send + 'a>>;
}

impl Cortex {
    pub(crate) fn identity(&self) -> &str {
        todo!()
    }

    pub(crate) async fn think(&self, messages: &[Message]) -> Thought {
        let _ = messages;
        todo!()
    }
}