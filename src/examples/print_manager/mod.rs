use std::fmt::Display;

use crate::ds::queue::{Queue, QueueTrait};

pub trait PrintManagerTrait<T> {
    fn new() -> Self;
    fn queue_print_job(&mut self, document: T);
    fn run(&mut self);
}

pub trait PrintManagerTraitPriv<T> {
    fn print(document: T);
    fn initialize() -> Queue<T>;
}

pub struct PrintManager<T> {
    queue: Queue<T>
}

impl<T: Display> PrintManagerTraitPriv<T> for PrintManager<T> {
    fn initialize() -> Queue<T> {
        Queue::new()
    }

    fn print(document: T) {
        println!("{}", document);
    }
}

impl<T: Display> PrintManagerTrait<T> for PrintManager<T> {
    fn new() -> Self {
        Self {
            queue: Self::initialize()
        }
    }

    fn queue_print_job(&mut self, document: T) {
        self.queue.enqueue(document);
    }

    fn run(&mut self) {
        while let Some(document) = self.queue.dequeue() {
            if self.queue.read().is_some() {
                print!("[{}] - ", document);
            } else {
                print!("[{}]", document);
            }
        }
        println!("");
    }
}