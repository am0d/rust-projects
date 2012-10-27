extern mod std;

/* For future reference, if necessary (from pcwalton, #rust, 2012-10-17 20:38)
   basically to allocate something in the same lifetime as yourself, you need to use an arena
   take in the arena, and then allocate into it. for example:
   impl Foo { fn make_foo(&self, arena_self_is_in: &self/Arena) -> &self/Foo { ... } }
   you need to pass in the arena that self is in
   so that the function can allocate into it
*/

struct Tree {
    mut leftTree: Option<~Tree>,
    mut rightTree: Option<~Tree>,
    key: int
}

impl Tree {
    fn insert(&self, i: int) {
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

    fn insertTree(&self, t: &self/Tree) {
        if self.key == t.key {
            let mut node = None;
            node <-> t.leftTree;
            match node {
                Some (ref lt) => {
                    let mut leftTree = None;
                    leftTree <-> self.leftTree;
                    match leftTree {
                        Some (ref slt) => slt.insertTree(*lt),
                        None => leftTree = Some (copy *lt)
                    };
                    leftTree <-> self.leftTree;
                },
                None => ()
            };
            node <-> t.rightTree;
            match node {
                Some (ref rt) => {
                    let mut rightTree = None;
                    rightTree <-> self.rightTree;
                    match rightTree {
                        Some (ref srt) => srt.insertTree(*rt),
                        None => rightTree = Some (copy *rt)
                    };
                    rightTree <-> self.rightTree;
                },
                None => ()
            };
        }
    }

    fn delete(&self, key: int) {
        assert self.key != key;

        let mut node = None;
        node <-> self.leftTree;
        match node {
            Some (ref t) => {
                if t.key == key {
                    // self.leftTree --> top node is the one to delete

                    // need to remove the left tree root
                    let mut tlt = None;
                    tlt <-> t.leftTree;
                    self.leftTree = tlt;

                    // and the right tree root
                    let mut trt = None;
                    trt <-> t.rightTree;
                    match trt {
                        Some (ref tr) => {
                            // the deleted node has a right tree
                            // we need to insert that into self.rightTree
                            let mut srt = None;
                            srt <-> self.rightTree;
                            match srt {
                                Some (ref rt) => rt.insertTree(*tr),
                                None => self.rightTree = tlt
                            };
                        },
                        None => {
                            // the deleted node didn't have a right tree
                            // we don't need to do anything here
                            ()
                        }
                    };

                    return ();
                }
                else {
                    // don't remove the left tree, put it back instead (later)
                    if t.key < key {
                        t.delete(key);
                    }
                }
            },
            None => {
                // self.leftTree doesn't exists
                ()
            }
        };
        node <-> self.leftTree; 
            
        node = None;
        node <-> self.rightTree;
        match node {
            Some (ref t) => {
                if t.key == key {
                    // self.rightTree --> top node is the one to delete

                    // need to remove the right tree root
                    let mut trt = None;
                    trt <-> t.rightTree;
                    self.rightTree = trt;

                    // and the left tree root
                    let mut tlt = None;
                    tlt <-> t.leftTree;
                    match tlt {
                        Some (ref tl) => {
                            // the deleted node has a left tree
                            // we need to insert that into self.leftTree
                            let mut slt = None;
                            slt <-> self.leftTree;
                            match slt {
                                Some (ref lt) => lt.insertTree(*tl),
                                None => self.leftTree = tlt
                            };
                        },
                        None => {
                            // the deleted node didn't have a right tree
                            // we don't need to do anything here
                            ()
                        }
                    };

                    return ();
                    // need to remove the right tree root
                    //let mut trt = None;
                    //trt <-> t.rightTree;
                    //self.rightTree = trt;
                    //return ();
                }
                else {
                    // don't remove the right tree, put it back instead
                    if t.key > key {
                        t.delete(key);
                    }
                }
            },
            None => {
                ()
            }
        };
        node <-> self.rightTree; 
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

impl Tree: ToStr {
    pure fn to_str() -> ~str {
        return fmt!("%d", self.key);
    }
}

fn fillTreeRandom (tree: &Tree, numNodes: int) {
    let r = rand::Rng();
    for int::range(0, numNodes) |_index| {
        tree.insert(r.gen_int_range(0, numNodes));
    }
}

fn fillTreeRec (tree: &Tree, key: int, depth: int) {
    tree.insert(key);
    if depth >= 0 {
        let div = float::pow_with_uint(2, depth as uint) as int;
        //io::println(fmt!("key: %d, depth: %d, div: %d", key, depth, div));
        fillTreeRec(tree, key - div, depth - 1);
        fillTreeRec(tree, key + div, depth - 1);
    }
}

fn main () {
    let root: Tree = Tree {leftTree: None, rightTree: None, key: 5};

    root.insert(8);
    //fillTreeRandom(&root, 1000);
    fillTreeRec(&root, 8, 2);

    root.print();
    io::println("");

    if root.contains(4) {
        io::println("Contains 4")
    }
    else {
        io::println("Doesn't contain 4")
    }

    root.delete(4);
    root.delete(8);

    if root.contains(4) {
        io::println("Still contains 4 now ...")
    }
    else {
        io::println("Doesn't contain 4 anymore")
    }
    root.print();
    io::println("");

    io::println(fmt!("Tree depth: %d", root.treeDepth()));

}
