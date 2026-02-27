mod trace;
use trace::Trace;

pub(crate) struct Traces {
    traces: Vec<Trace>,
}

impl Traces {
    pub(crate) fn new() -> Self {
        Self { traces: Vec::new() }
    }

    pub(crate) fn imprint() -> Trace {
        todo!()
    }

    pub(crate) fn store(&mut self, trace: Trace) {
        let _ = trace;
        todo!()
    }

    pub(crate) fn recall(&self) -> Vec<Trace> {
        todo!()
    }

    pub(crate) fn dissolve(&mut self, trace_id: usize) {
        let _ = trace_id;
        todo!()
    }
}
