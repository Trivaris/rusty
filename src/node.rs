type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value: value, next: None }
    }

    pub fn get() -> T {
        self.value
    }
}