// Tool Search Tool scaffold: Tools discovered on-demand instead of preloaded
// Tool Use Examples scaffold: Concrete examples beyond schema definitions

pub const NATIVE_TOOL_SEARCH: &str = "search_tools";

#[derive(Clone, Debug)]
pub struct SearchToolsRequest {
    pub query: String,
    pub limit: usize,
}

#[derive(Clone, Debug)]
pub struct SearchToolsResultItem {
    pub name: String,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct SearchToolsResponse {
    pub matches: Vec<SearchToolsResultItem>,
}

#[derive(Clone, Debug)]
pub struct ToolIndexItem {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ToolExample {
    pub tool_name: String,
    pub example_input_json: String,
    pub why_this_example: String,
}

#[derive(Default)]
pub struct ToolRegistry {
    pub index: Vec<ToolIndexItem>,
    pub examples: Vec<ToolExample>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_tools(&self, query: &str) -> Vec<ToolIndexItem> {
        // TODO(Tool Search Tool): Implement very simple match first (name/description contains query).
        // TODO(Tool Search Tool): Later add better ranking (tags, synonyms, embeddings, etc).
        // QUESTION: Should search return top 3 or top 5 tools by default?

        self.index
            .iter()
            .filter(|item| {
                item.name.contains(query)
                    || item.description.contains(query)
                    || item.tags.iter().any(|tag| tag.contains(query))
            })
            .cloned()
            .collect()
    }

    pub fn call_native_search_tools(&self, request: SearchToolsRequest) -> SearchToolsResponse {
        // search_tools is a native/standard tool surface exposed to the LLM.
        // Internally it calls the registry implementation above.
        //
        // TODO(Tool Search Tool): Decide whether to normalize casing before matching.
        // TODO(Tool Search Tool): Decide whether to cap max limit (ex: max 5).
        // QUESTION: Should we include score/confidence in response now, or later?

        let mut found = self.search_tools(&request.query);
        if found.len() > request.limit {
            found.truncate(request.limit);
        }

        let matches = found
            .into_iter()
            .map(|item| SearchToolsResultItem {
                name: item.name,
                reason: "matched by name/description/tags".to_string(),
            })
            .collect();

        SearchToolsResponse { matches }
    }

    pub fn examples_for(&self, tool_name: &str) -> Vec<ToolExample> {
        // TODO(Tool Use Examples): Keep 1-3 examples per tool for now.
        // TODO(Tool Use Examples): Filter out stale examples when tool schema changes.
        // QUESTION: Do you want examples in plain text JSON or serde_json::Value later?

        self.examples
            .iter()
            .filter(|example| example.tool_name == tool_name)
            .cloned()
            .collect()
    }

    pub fn build_prompt_hint_for_tool(&self, tool_name: &str) -> String {
        // TODO(Tool Use Examples): Build short prompt text with examples.
        // TODO(Tool Use Examples): Include this hint only when the tool is selected.

        let examples = self.examples_for(tool_name);
        if examples.is_empty() {
            return String::new();
        }

        format!(
            "Tool examples for {}: {} sample(s)",
            tool_name,
            examples.len()
        )
    }
}
