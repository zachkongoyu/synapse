pub(crate) struct Thought {
    id: u64,
    parent_id: Option<u64>,
    content: String,
}

impl Thought {
    const MAX_CONTENT_LEN: usize = 280;
    const EMPTY_CONTENT_FALLBACK: &str = "...";

    pub(in crate::agent::brain::cortex) fn new(
        id: u64,
        parent_id: Option<u64>,
        content: impl Into<String>,
    ) -> Self {
        let _ = (id, parent_id, content.into());
        todo!()
    }

    pub(crate) fn id(&self) -> u64 {
        todo!()
    }

    pub(crate) fn parent_id(&self) -> Option<u64> {
        todo!()
    }

    pub(crate) fn content(&self) -> &str {
        todo!()
    }

    fn normalize_content(content: String) -> String {
        let _ = content;
        todo!()
    }

}
