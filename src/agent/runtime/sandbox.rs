#[derive(Clone, Debug)]
pub struct Script {
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct SandboxOutput {
    pub final_text: String,
}

pub struct Sandbox;

impl Sandbox {
    pub fn run(&self, _script: &Script) -> SandboxOutput {
        // TODO: Execute script in isolated environment.
        // TODO: Expose a tool bridge API to the script.
        // TODO: Limit what script can access based on RuntimePolicy.
        // TODO: Return only final summary text.

        // QUESTION: Which language will your first script use (Python or Rust)?
        // QUESTION: Should timeout be hard-coded first, then configurable later?
        SandboxOutput {
            final_text: String::new(),
        }
    }
}
