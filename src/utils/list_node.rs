pub struct ListNode<T> {
    next: Option<Box<ListNode<T>>>,
    val: T,
}

impl<T> ListNode<T> {
    pub fn chain(&mut self, next_node: ListNode<T>) {
        self.next = Some(Box::new(next_node));
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn next(&self) -> Option<&ListNode<T>> {
        self.next.as_deref()
    }
}

impl<T> Iterator for ListNode<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current_value = std::mem::replace(&mut self.val, unsafe { std::mem::zeroed() });

        if let Some(next_node) = self.next.take() {
            *self = *next_node;
            Some(current_value)
        } else {
            None
        }
    }
}
