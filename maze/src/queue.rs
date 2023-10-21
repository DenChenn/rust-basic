pub struct Queue<T> {
    storage: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        return Queue { storage: Vec::new() }
    }
    pub fn push_back(&mut self, item: T) {
        self.storage.push(item)
    }
    pub fn pop_front(&mut self) -> T {
        self.storage.remove(0)
    }
    pub fn is_empty(&self) -> bool {
        return self.storage.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        return self.storage.first();
    }
}
