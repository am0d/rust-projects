#![feature(macro_rules)]

struct Tree<T> {
    root: Option<Box<Node<T>>>
}
pub struct Node<T> {
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
    key: T
}

pub struct PreOrderTreeIterator<'t, T> {
    stack: Vec<&'t Box<Node<T>>>
}

impl<'t, T:Ord+Eq> Iterator<&'t Box<Node<T>>> for PreOrderTreeIterator<'t, T> {
    fn next(&mut self) -> Option<&'t Box<Node<T>>> {
        match self.stack.pop() {
            Some(ref n) => {
                match n.left_child {
                    Some(ref n) => {
                        self.stack.push(n);
                    },
                    _ => ()
                };
                match n.right_child {
                    Some(ref n) => {
                        self.stack.push(n);
                    },
                    _ => ()
                };
                Some(*n)
            },
            None => None
        }
    }
}


impl<T:Ord+Eq> Node<T> {
    pub fn new (nodeKey: T) -> Node<T> {
        Node {
            left_child: None,
            right_child: None,
            key: nodeKey
        }
    }

    pub fn add_child_node(&mut self, new_node: Box<Node<T>>) {
        if new_node.key > self.key {
            self.add_right_child(new_node);
        }
        else {
            self.add_left_child(new_node);
        }
    }

    fn add_right_child(&mut self, new_node: Box<Node<T>>) {
        match self.right_child {
            None => {
                self.right_child = Some(new_node);
            },
            Some(ref mut right_child) => {
                right_child.add_child_node(new_node);
            }
        }
    }

    fn add_left_child(&mut self, new_node: Box<Node<T>>) {
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
        let new_node = box Node::new(value);
        self.insert_node(new_node);
    }

    pub fn insert_node (&mut self, new_node: Box<Node<T>>) {
        match self.root {
            None => {
                self.root = Some(new_node);
            },
            Some(ref mut n) => {
                n.add_child_node(new_node);
            }
        }
    }

    pub fn visit_pre_order<'n>(&'n self) -> PreOrderTreeIterator<'n, T> {
        match self.root {
            Some(ref n) => {
                PreOrderTreeIterator {
                    stack: vec!(&*n)
                }
            },
            _ => {
                PreOrderTreeIterator {
                    stack: vec!()
                }
            }
        }
    }
}

fn main () {
    let mut my_tree: Tree<int> = Tree::new();

    for key in vec!(3, 2, 4, 0, 8, 11, 18, 22, 16, 12, 7, 10).iter() {
        my_tree.insert_value(*key);
    }

    for node in my_tree.visit_pre_order() {
        println!("{}", node.key);
    }
}
