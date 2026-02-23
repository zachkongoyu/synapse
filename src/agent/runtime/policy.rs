#[derive(Clone, Debug)]
pub enum RuntimeAllowance {
    ApprovedOnly(Vec<String>),
    Everything,
}

#[derive(Clone, Debug)]
pub struct RuntimePolicy {
    pub allowance: RuntimeAllowance,
}

impl RuntimePolicy {
    pub fn basic_locked_down() -> Self {
        Self {
            allowance: RuntimeAllowance::ApprovedOnly(vec![]),
        }
    }

    pub fn is_tool_allowed(&self, tool_name: &str) -> bool {
        match &self.allowance {
            RuntimeAllowance::Everything => true,
            RuntimeAllowance::ApprovedOnly(approved_tools) => approved_tools.iter().any(|t| t == tool_name),
        }
    }

    pub fn allow_all(&mut self) {
        self.allowance = RuntimeAllowance::Everything;
    }

    pub fn set_approved_tools(&mut self, tools: Vec<String>) {
        self.allowance = RuntimeAllowance::ApprovedOnly(tools);

        // TODO: Validate tool names exist in your registry.
        // QUESTION: Should unknown tool names be ignored or treated as config errors?
    }
}
