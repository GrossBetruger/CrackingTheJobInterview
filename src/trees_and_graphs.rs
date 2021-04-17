

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

    pub fn leaf_node(data: T) -> Node<T> {
        Node {
            data,
            children: vec![]
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

fn traverse_node_in_order<T: std::cmp::PartialEq + std::fmt::Display>(node: &Node<T>){

    for i in 0..node.children.len() {
        traverse_node_in_order(&node.children[i]);
        println!("{}", &node.children[i].data);
    }
    println!("{}", node.data);
    // node.data == search_term
}
    
#[cfg(test)]
mod tests {
    use crate::trees_and_graphs::{Node, Tree, traverse_node_in_order};

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
        assert_eq!("str", str_tree.root.data);

        let nodes = Node::new(1, vec![Node::leaf_node(2), Node::leaf_node(4)]);
        traverse_node_in_order(&nodes);
        // assert_eq!(true, traverse_node_in_order(&nodes, 1));
        // assert_eq!(true, traverse_node_in_order(&nodes, 2));
    }
}
