

struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>
}

struct BinaryNode<T: std::fmt::Display> {
    pub data: T,
    pub left: Box<Option<BinaryNode<T>>>,
    pub right: Box<Option<BinaryNode<T>>>
}

struct Tree<T> {
    pub root: Node<T>
}

struct BinaryTree<T: std::fmt::Display> {
    pub root: BinaryNode<T>
}

impl <T: std::fmt::Display> BinaryNode<T> {
    pub fn new(data: T, left: Box<Option<BinaryNode<T>>>, right: Box<Option<BinaryNode<T>>>)
        -> BinaryNode<T> {
        BinaryNode {
            data,
            left,
            right
        }
    }
}

impl <T: std::fmt::Display> BinaryTree<T> {
    pub fn new(root: BinaryNode<T>) -> BinaryTree<T> {
        BinaryTree {
            root
        }
    }
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
    use crate::trees_and_graphs::{Node, Tree, traverse_node_in_order, BinaryTree, BinaryNode};
    use std::collections::HashMap;

    #[test]
    fn test_tree_construction() {
        let binary_root_node = BinaryNode::new(10,
                                                                Box::new(Some(BinaryNode::new(5,
                                                                                                    Box::new(Some(BinaryNode::new(1111,
                                                                                                                                            Box::new(None), Box::new(None))),
                                                                                                    ), Box::new(None)))), Box::new(None));

        let binary_tree = BinaryTree::new(binary_root_node);

        println!("{:?}", binary_tree.root.left.unwrap().left.unwrap().data);

        // let root = Node::new(
        //     3,
        //     vec![Node::new(2, vec![]), Node::new(0, vec![])]
        // );
        //
        // let tree = Tree::new(root);
        //
        // let root = Node::new("str",
        //                     vec![Node::new("inner str", vec![])]);
        //
        // let str_tree = Tree::new(root);
        //
        // assert_eq!(3, tree.root.data);
        // assert_eq!(2, tree.root.children[0].data);
        // assert_eq!(0, tree.root.children[1].data);
        //
        // assert_eq!("inner str", str_tree.root.children[0].data);
        // assert_eq!("str", str_tree.root.data);
        //
        // let nodes = Node::new(1, vec![Node::leaf_node(2), Node::leaf_node(4)]);
        // traverse_node_in_order(&nodes);
        // assert_eq!(true, traverse_node_in_order(&nodes, 1));
        // assert_eq!(true, traverse_node_in_order(&nodes, 2));
    }
}
