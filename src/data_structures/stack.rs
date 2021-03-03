pub struct MyStack<T> {
    data: Vec<T>,
}

pub trait StackBasics<T> {
    fn push(&self, x: T);
    fn pop(&self) -> T;
    fn top(&self) -> T;
    fn empty(&self) -> T;
}

impl<T> MyStack<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}