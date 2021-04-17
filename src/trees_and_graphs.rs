

struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>
}

struct Tree<T> {
    pub root: Node<T>
}


impl<T> Node<T> {
    pub fn new(data: T, children: Vec<Node<T>>) -> Node<T> {
        Node {
            data,
            children
        }
    }
}

impl<T> Tree<T> {
    pub fn new(root: Node<T>) -> Tree<T> {
        Tree {
            root
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trees_and_graphs::{Node, Tree};

    #[test]
    fn test_tree_construction() {
        let root = Node::new(
            3,
            vec![Node::new(2, vec![]), Node::new(0, vec![])]
        );

        let tree = Tree::new(root);

        let root = Node::new("str",
                            vec![Node::new("inner str", vec![])]);

        let str_tree = Tree::new(root);

        assert_eq!(3, tree.root.data);
        assert_eq!(2, tree.root.children[0].data);
        assert_eq!(0, tree.root.children[1].data);

        assert_eq!("inner str", str_tree.root.children[0].data);
        assert_eq!("str", str_tree.root.data)
    }
}
