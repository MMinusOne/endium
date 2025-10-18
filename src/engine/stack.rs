pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn peek_at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}
