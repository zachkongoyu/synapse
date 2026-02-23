use serde_json::Value;
use crate::agent::brain::memory::Memory;
use crate::agent::runtime::ExecutionMode;

struct Cortex {
    mind: Box<dyn Provider>,
    identity: String,
    tool_schemas: Vec<Value>,
}

enum Thought {}

impl Cortex {
    fn think(&self, memory: &Memory) -> Option<Thought> {
        let messages = memory.read(None);

        // TODO(Tool Search Tool): Start with only core tools loaded.
        // TODO(Tool Search Tool): Always include native tool "search_tools".
        // TODO(Tool Search Tool): If model asks for more, call search_tools(query).
        // TODO(Tool Search Tool): Load only matched full tool schemas, not everything.
        // TODO(Tool Search Tool): In next LLM turn, inject only those matched schemas.
        // QUESTION: Which tools are your "always loaded" core set?

        // TODO(Tool Use Examples): Before calling a selected tool,
        // inject 1-3 tool examples as short prompt hints.
        // QUESTION: Should examples be shown in system prompt or tool description?

        // TODO(Programmatic Tool Calling (PTC)): Add a mode decision here:
        // - ExecutionMode::Interactive for regular small tasks
        // - ExecutionMode::Programmatic for script-based orchestration in runtime
        //
        // TODO(Programmatic Tool Calling (PTC)): In Programmatic mode:
        // 1) Ask the model for a script (not one-tool-per-turn calls).
        // 2) Send script to RuntimeOrchestrator.
        // 3) Runtime handles many tool calls internally.
        // 4) Only memorize final compact summary.
        let _mode = ExecutionMode::Interactive;

        let response = self.mind.process(messages);
        None
    }
}