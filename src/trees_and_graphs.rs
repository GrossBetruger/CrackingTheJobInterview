

struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>
}

struct Tree<T> {
    pub root: Node<T>
}


#[cfg(test)]
mod tests {
    use crate::trees_and_graphs::{Node, Tree};

    #[test]
    fn new_test() {
        let tree = Tree {
            root: Node {
                data: 3,
                children: vec![Node{data: 2, children: vec![]}, Node{data: 0, children: vec![]}]
            }
        };

        let str_tree = Tree {
            root: Node {
                data: "str",
                children: vec![Node{data: "inner str", children: vec![]}]
            }
        };

        assert_eq!(2, tree.root.children[0].data);
        assert_eq!(0, tree.root.children[1].data);

        assert_eq!("inner str", str_tree.root.children[0].data);
        assert_eq!("str", str_tree.root.data)
    }
}
