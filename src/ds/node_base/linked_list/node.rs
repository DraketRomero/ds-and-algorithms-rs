
#[derive(Debug)]
pub enum Node<T> {
    Cons(T, Box<Node<T>>),
    None
}