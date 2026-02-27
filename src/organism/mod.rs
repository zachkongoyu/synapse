mod membrane;
mod nucleus;

use membrane::Membrane;
use nucleus::Nucleus;

pub struct Organism {
    nucleus: Nucleus,
    membrane: Membrane,
    id: u64,
    name: String,
    birthed_at: String,
}

impl Organism {
    pub fn new(id: u64, name: String, birthed_at: String) -> Self {
        Self {
            nucleus: Nucleus::new(),
            membrane: Membrane::new(),
            id,
            name,
            birthed_at,
        }
    }

    pub async fn flow(&self) {
        self.nucleus.protoplasm.flow().await;
    }

    pub async fn extend(&self) {
        self.membrane.extend().await;
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
