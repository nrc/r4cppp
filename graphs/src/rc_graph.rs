
use std::rc::Rc;
use std::mem;
use std::collections::HashSet;

struct Node {
    datum: &'static str,
    next: Vec<Rc<Node>>,
}

impl Node {
    fn new(datum: &'static str) -> Rc<Node> {
        Rc::new(Node {
            datum: datum,
            next: Vec::new(),
        })
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
            n.traverse(f, seen);
        }
    }

    fn first(&self) -> &Node {
        &self.next[0]
    }
}

fn foo(node: &Node) {
    println!("foo: {}", node.datum);
}

fn init() -> Rc<Node> {
    let root = Node::new("A");

    let b = Node::new("B");
    let c = Node::new("C");
    let d = Node::new("D");
    let e = Node::new("E");
    let f = Node::new("F");

    unsafe {
        let mut_root: &mut Node = mem::transmute(&*root);
        mut_root.next.push(b.clone());
        mut_root.next.push(c.clone());
        mut_root.next.push(d.clone());

        let mut_c: &mut Node = mem::transmute(&*c);
        mut_c.next.push(e.clone());
        mut_c.next.push(f.clone());
        mut_c.next.push(root.clone());
    }

    root
}

pub fn main() {
    let g = init();
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    foo(g.first());
}
