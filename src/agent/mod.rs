mod brain;
mod body;

use brain::Brain;
use body::Body;

pub struct Agent {
    brain: Brain,
    body: Body,
    // Metadata
    id: u64,
    name: String,
    created_at: String, // Birthday.
}

impl Agent {
    pub fn new(id: u64, name: String, created_at: String) -> Self {
        Self {
            brain: Brain::new(),
            body: Body::new(),
            id,
            name,
            created_at,
        }
    }

    pub async fn think(&self) {
        // Placeholder for thinking process
        self.brain.cortex.think().await;
    }

    pub async fn act(&self) {
        // Placeholder for action process
        self.body.act().await;
    }
}



trait Rename {
    fn rename(&mut self, new_name: &str) {
        let _ = new_name;
        todo!()
    }
}

trait Age {
    fn age(&self) -> u32 {
        todo!()
    }
}