

struct Node {
    pub data: u32,
    pub children: Vec<Node>
}

struct Tree {
    pub root: Node
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

        assert_eq!(2, tree.root.children[0].data);
        assert_eq!(0, tree.root.children[1].data);
    }
}
