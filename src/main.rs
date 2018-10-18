type NodeWrapper = Option<Box<Node>>;

struct Node {
    left: NodeWrapper,
    right: NodeWrapper,
    weight: i64,
}

impl Node {
    fn new(weight: i64) -> Self {
        Node {
            left: None,
            right: None,
            weight,
        }
    }
}

struct Tree {
    root: NodeWrapper,
}

impl Tree {
    fn new() -> Self {
        Tree {
            root: None,
        }
    }

    fn insert_helper(node: &mut Node, wt: i64) {
        if wt < node.weight {
            match node.left {
                Some(ref mut n) => {
                    Tree::insert_helper(n, wt);
                },
                None => {
                    node.left = Some(Box::new(Node::new(wt)));
                }
            }
        } else if wt > node.weight {
            match node.right {
                Some(ref mut n) => {
                    Tree::insert_helper(n, wt);
                },
                None => {
                    node.right = Some(Box::new(Node::new(wt)));
                }
            }
        } else {

        }
    }

    fn insert(&mut self, wt: i64) {
        match self.root {
            Some(ref mut n) => {
                Tree::insert_helper(n, wt);
            },
            None => {
                self.root = Some(Box::new(Node::new(wt)));
            }
        }
    }

    fn display_helper(node: &Node) {
        match node.left {
            Some(ref n) => {
                Tree::display_helper(n);
                println!("{}", n.weight);
            },
            None => {
            }
        }
        match node.right {
            Some(ref n) => {
                Tree::display_helper(n);
                println!("{}", n.weight);
            },
            None => {
            }
        }
    }

    fn display(&self) {
        match self.root {
            Some(ref n) => {
                Tree::display_helper(n);
                println!("{}", n.weight);
            },
            None => {
                println!("Empty tree.");
            }
        }
    }

    fn search_helper(node: &Node, wt: i64) -> Option<&Node> {
        if wt < node.weight {
            match node.left {
                Some(ref n) =>  Tree::search_helper(n, wt),
                None => None
            }
        } else if wt > node.weight {
            match node.right {
                Some(ref n) => Tree::search_helper(n, wt),
                None => None
            }
        } else {
            Some(node)
        }
    }

    fn search(&self, wt: i64) -> Option<&Node> {
        match self.root {
            Some(ref n) => {
                Tree::search_helper(n, wt)
            },
            None => None
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(7);
    tree.insert(2);
    tree.insert(3);
    tree.insert(12);
    tree.display();
    match tree.search(12) {
        Some(n) => println!("{} is found", n.weight),
        None => println!("7 not found")
    }
    match tree.search(6) {
        Some(n) => println!("{} is found", n.weight),
        None => println!("6 not found")
    }
}
