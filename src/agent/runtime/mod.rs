pub mod policy;
pub mod sandbox;

use self::policy::RuntimePolicy;
use self::sandbox::Script;

pub enum ExecutionMode {
    Interactive,
    Programmatic,
}

pub struct Runtime {
    pub mode: ExecutionMode,
    pub policy: RuntimePolicy,
}

impl Runtime {
    pub fn new(mode: ExecutionMode, policy: RuntimePolicy) -> Self {
        Self {
            mode,
            policy,
        }
    }

    pub fn set_mode(&mut self, mode: ExecutionMode) {
        self.mode = mode;
    }

    pub fn set_policy(&mut self, policy: RuntimePolicy) {
        self.policy = policy;
    }

    pub fn run_programmatic(&self, _script: &Script) {
        // TODO: Ask model to generate ONE script for the full task.
        // TODO: Run the script in sandbox.
        // TODO: Let script call many tools internally.
        // TODO: Send only final result back to model memory.

        // QUESTION: When should we switch to Programmatic mode?
        // QUESTION: Should denied tools stop the script or return an error value to the script?
    }
}
