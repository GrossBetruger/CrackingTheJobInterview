use std::collections::{LinkedList, HashSet};

struct Node<T> {
    next: Box<Option<Node<T>>>,
    data: T

}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            next: Box::new(None),
            data
        }
    }

    pub fn add(&mut self, data: T) {
        let node = Node::new(data);
        self.next = Box::new(Option::from(node));
    }
}


#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use crate::linked_lists::{Node};

    #[test]
    fn new_test() {
        let mut list = Node::new(7);
        list.add(10);
        list.add(11);
        println!("{}", list.data);
        // println!("{}", list.next.unwrap().data);
        println!("{}", list.next.unwrap().data);
        // assert_eq!(10, list.next.data)
    }
}
