extern mod std;

/* For future reference, if necessary (from pcwalton, #rust, 2012-10-17 20:38)
   basically to allocate something in the same lifetime as yourself, you need to use an arena
   take in the arena, and then allocate into it. for example:
   impl Foo { fn make_foo(&self, arena_self_is_in: &self/Arena) -> &self/Foo { ... } }
   you need to pass in the arena that self is in
   so that the function can allocate into it
*/

struct Tree/&{
    mut leftTree: Option<~Tree>,
    mut rightTree: Option<~Tree>,
    key: int
}

impl Tree {
    fn insert(&self, i: int) -> () {
        let mut node = None;
        if i > self.key {
            node <-> self.rightTree;
            match node {
                Some(ref t) => t.insert(i),
                None => node = Some(~Tree {leftTree: None, rightTree: None, key: i})
            }
            node <-> self.rightTree;
        }
        if i < self.key {
            node <-> self.leftTree;
            match node {
                Some (ref t) => t.insert(i),
                None => node = Some(~Tree {leftTree: None, rightTree: None, key: i})
            }
            node <-> self.leftTree;
        }

    }

    pure fn treeDepth (&self) -> int {
        let leftCount = match self.leftTree {
            Some (ref t) => t.treeDepth(),
            None => 0
        };
        let rightCount = match self.rightTree {
            Some (ref t) => t.treeDepth(),
            None => 0
        };
        1 + int::max(leftCount, rightCount)
    }

    pure fn contains(i: int) -> bool {
        if i == self.key {
            return true;
        }

        if i > self.key {
            match self.rightTree {
                Some(~t) => t.contains(i),
                None => false
            }
        }
        else {
            match self.leftTree {
                Some(~t) => t.contains(i),
                None => false
            }
        }
    }

    fn print() {
        let mut node = None;
        node <-> self.leftTree;
        match node {
            Some(ref t) => t.print(),
            None => ()
        };
        node <-> self.leftTree;

        io::print(fmt!("%d ", self.key));

        node <-> self.rightTree;
        match node {
            Some(ref t) => t.print(),
            None => ()
        };
        node <-> self.rightTree;
    }

}

fn fillTree (tree: &Tree, numNodes: int) {
    let r = rand::Rng();
    for int::range(0, numNodes) |_index| {
        tree.insert(r.gen_int_range(0, numNodes));
    }
}

fn main () {
    let root: Tree = Tree {leftTree: None, rightTree: None, key: 5};

    root.insert(8);
    fillTree(&root, 1000);

    root.print();
    io::println("");

    if root.contains(4) {
        io::println("Contains 4")
    }
    else {
        io::println("Doesn't contain 4")
    }

    root.insert(4);

    if root.contains(4) {
        io::println("Contains 4 now ...")
    }
    else {
        io::println("Doesn't contain 4 yet")
    }

    io::println(fmt!("Tree depth: %d", root.treeDepth()));

}
