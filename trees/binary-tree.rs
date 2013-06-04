use std::Option;

struct Tree<T> {
    root: Option<~Node<T>>
}
pub struct Node<T> {
    left_child: Option<~Node<T>>,
    right_child: Option<~Node<T>>,
    key: T
}

impl<T:Ord> Node<T> {
    pub fn new (nodeKey: T) -> Node<T> {
        Node {
            left_child: None,
            right_child: None,
            key: nodeKey
        }
    }

    pub fn add_child_node(&mut self, new_node: ~Node<T>) {
        if new_node.key > self.key {
            self.add_right_child(new_node);
        }
        else {
            self.add_left_child(new_node);
        }
    }

    fn add_right_child(&mut self, new_node: ~Node<T>) {
        match self.right_child {
            None => {
                self.right_child = Some(new_node);
            },
            Some(ref mut right_child) => {
                right_child.add_child_node(new_node);
            }
        }
    }

    fn add_left_child(&mut self, new_node: ~Node<T>) {
        match self.left_child {
            None => {
                self.left_child = Some(new_node);
            },
            Some(ref mut left_child) => {
                left_child.add_child_node(new_node);
            }
        }
    }

    pub fn has_children(node: &Node<T>) -> bool {
        match node.left_child {
            Some(_) => true,
            _ => {
                match node.right_child {
                    Some(_) => true,
                    _ => false
                }
            }
        }
    }
}

pub impl<T:Ord> Tree<T> {
    pub fn new () -> Tree<T> {
        Tree {
            root: None
        }
    }

    fn insert_value (&mut self, value: T) {
        let new_node = ~Node::new(value);
        self.insert_node(new_node);
    }

    fn insert_node (&mut self, new_node: ~Node<T>) {
        match self.root {
            None => {
                self.root = Some(new_node);
            },
            Some(ref mut n) => {
                n.add_child_node(new_node);
            }
        }
    }
}

fn main () {
    let mut myTree: Tree<int> = Tree::new();

    myTree.insert_value(3);
    myTree.insert_value(2);
    myTree.insert_value(4);
}
