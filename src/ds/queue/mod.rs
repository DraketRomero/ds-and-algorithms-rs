pub trait QueueTrait<T> {
    fn new() -> Self;
    fn enqueue(&mut self, element: T);
    fn dequeue(&mut self) -> Option<T>;
    fn read(&self) -> Option<&T>;
}

pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn enqueue(&mut self, element: T) {
        self.data.push(element);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn read(&self) -> Option<&T> {
        self.data.first()
    }
}
