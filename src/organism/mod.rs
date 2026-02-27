mod membrane;
mod flux;
mod trace;

use membrane::Membrane;
use flux::Flux;
use trace::TraceField;

pub struct Organism {
    flux: Flux,
    trace_field: TraceField,
    membrane: Membrane,
    id: u64,
    name: String,
    birthed_at: String,
}

impl Organism {
    pub fn new(id: u64, name: String, birthed_at: String) -> Self {
        Self {
            flux: Flux::new(),
            trace_field: TraceField::new(),
            membrane: Membrane::new(),
            id,
            name,
            birthed_at,
        }
    }

    pub async fn flow(&self) {
        self.flux.flow().await;
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
