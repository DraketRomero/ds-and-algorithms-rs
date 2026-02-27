pub trait StackTrait<T> {
    fn new() -> Self;
    fn push(&mut self, new_element: T);
    fn pop(&mut self) -> Option<T>;
    fn read(&self) -> Option<&T>;
}

pub struct Stack<T> {
    data: Vec<T>
}

impl<T> StackTrait<T> for Stack<T>{
    fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    fn push(&mut self, new_element: T) {
        self.data.push(new_element);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn read(&self) -> Option<&T> {
        self.data.last()
    }
}

