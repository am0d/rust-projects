pub struct TreeIterator<'tree, T> {
    stack: Vec<&'tree T>
}

impl<'tree, T:Node> Iterator<&'tree T> for TreeIterator<'tree, T> {
    fn next(& mut self) -> Option<&'tree T> {
        match self.stack.pop() {
            Some(n) => {
                match n.left_child {
                    Some(n) => {
                        self.stack.push(&*n);
                    },
                    _ => ()
                };
                match n.right_child {
                    Some(n) => {
                        self.stack.push(&*n);
                    },
                    _ => ()
                };
                Some(n)
            },
            None => None
        }
    }
}
