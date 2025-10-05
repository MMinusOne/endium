use std::sync::OnceLock;

#[derive()]
pub struct Heap {}

static INSTANCE: OnceLock<Heap> = OnceLock::new();

impl Heap {
    pub fn new() -> Self {
        Self {}
    }

    pub fn instance() -> &'static Self {
        INSTANCE.get_or_init(Self::new)
    }
}
