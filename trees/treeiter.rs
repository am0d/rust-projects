pub struct TreeIterator<'tree, T> {
    stack: ~[&'tree T]
}

impl<'tree, T> Iterator<&'tree T> for TreeIterator<'tree, T> {
    fn next<'a>(&'a mut self) -> Option<&'a T> {
        None
    }
}
