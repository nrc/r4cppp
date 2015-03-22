
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

struct Node {
    datum: &'static str,
    next: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(datum: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum: datum,
            next: Vec::new(),
        }))
    }

    fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
        where F: Fn(&'static str)
    {
        if seen.contains(&self.datum) {
            return;
        }
        f(self.datum);
        seen.insert(self.datum);
        for n in &self.next {
            n.borrow().traverse(f, seen);
        }
    }

    fn first(&self) -> Rc<RefCell<Node>> {
        self.next[0].clone()
    }
}

fn foo(node: &Node) {
    println!("foo: {}", node.datum);
}

fn init() -> Rc<RefCell<Node>> {
    let root = Node::new("A");

    let b = Node::new("B");
    let c = Node::new("C");
    let d = Node::new("D");
    let e = Node::new("E");
    let f = Node::new("F");

    {
        let mut mut_root = root.borrow_mut();
        mut_root.next.push(b.clone());
        mut_root.next.push(c.clone());
        mut_root.next.push(d.clone());

        let mut mut_c = c.borrow_mut();
        mut_c.next.push(e.clone());
        mut_c.next.push(f.clone());
        mut_c.next.push(root.clone());
    }

    root
}

pub fn main() {
    let g = init();
    let g = g.borrow();
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    let f = g.first();
    foo(&*f.borrow());
}
