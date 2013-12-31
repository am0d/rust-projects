use std::Option;
use std::ptr;

use treeiter::TreeIterator;

pub mod treeiter;

struct Tree<T> {
    root: Option<~Node<T>>
}
pub struct Node<T> {
    left_child: Option<~Node<T>>,
    right_child: Option<~Node<T>>,
    key: T
}

impl<T:Ord+Eq> Node<T> {
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

    pub fn visit_in_order<'n>(&self, visitor: |&T| -> ()) -> TreeIterator<'n, T> {
        TreeIterator {
            stack: ~[]
        }
    }

    /*pub fn visit_in_order(&self, visitor: &fn(&T)) {
        match self.left_child {
            Some (ref n) => {
                n.visit_in_order(visitor);
            },
            _ => {}
        }
        
        visitor(&self.key);

        match self.right_child {
            Some (ref n) => {
                n.visit_in_order(visitor);
            },
            _ => {}
        }
    }*/

    /*pub fn search(&self, needle: T) -> Option<T> {
        if self.key == needle {
            return Some(self.key);
        }
        else if self.key < needle {
            match self.left_child {
                Some(ref n) => {
                    return n.search(needle);
                },
                _ => {
                    return None;
                }
            }
        }
        return None;
    }*/
}

impl<T:Ord+Eq> Tree<T> {
    pub fn new () -> Tree<T> {
        Tree {
            root: None
        }
    }

    pub fn insert_value (&mut self, value: T) {
        let new_node = ~Node::new(value);
        self.insert_node(new_node);
    }

    pub fn insert_node (&mut self, new_node: ~Node<T>) {
        match self.root {
            None => {
                self.root = Some(new_node);
            },
            Some(ref mut n) => {
                n.add_child_node(new_node);
            }
        }
    }

    pub fn visit_in_order (&self, visitor: |&T|->()) {
        match self.root {
            Some(ref n) => {
                //n.visit_in_order(visitor);
            },
            _ => {
            }
        }
    }
}

fn main () {
    let mut myTree: Tree<int> = Tree::new();

    myTree.insert_value(3);
    myTree.insert_value(2);
    myTree.insert_value(4);
    myTree.insert_value(0);

    myTree.visit_in_order(|n| {
                          println!("{:d}", *n);
                          });
}
