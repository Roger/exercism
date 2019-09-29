use std::fmt;

#[derive(Debug, Default)]
pub struct SimpleLinkedList<T>
where
    T: fmt::Debug,
{
    head: Option<Box<Node<T>>>,
    length: usize,
}

#[derive(Debug, Default)]
pub struct Node<T>
where
    T: fmt::Debug,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T>
where
    T: fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn push(&mut self, element: T) {
        self.length += 1;
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            None => None,
            Some(node) => {
                self.length -= 1;
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(value) => Some(&value.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T>
where
    T: fmt::Debug,
{
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut sll = Self::new();
        let mut current = &self.head;
        while let Some(item) = current {
            sll.push(item.data.clone());
            current = &item.next;
        }
        sll
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T>
where
    T: fmt::Debug,
{
    fn from(item: &[T]) -> Self {
        let mut sll = Self::new();
        item.iter().for_each(|i| sll.push(i.clone()));
        sll
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: fmt::Debug,
{
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut current = self.head;
        while let Some(item) = current {
            vec.insert(0, item.data);
            current = item.next;
        }
        vec
    }
}
