
use std::cell::UnsafeCell;
use std::collections::HashSet;
use arena::TypedArena;

struct Node<'a> {
    datum: &'static str,
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new<'b>(datum: &'static str, arena: &'b TypedArena<Node<'b>>) -> &'b Node<'b> {
        arena.alloc(Node {
            datum: datum,
            edges: UnsafeCell::new(Vec::new()),
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
        unsafe {
            for n in &(*self.edges.get()) {
                n.traverse(f, seen);
            }
        }
    }

    fn first(&'a self) -> &'a Node<'a> {
        unsafe {
            (*self.edges.get())[0]
        }
    }
}

fn foo<'a>(node: &'a Node<'a>) {
    println!("foo: {}", node.datum);
}

fn init<'a>(arena: &'a TypedArena<Node<'a>>) ->&'a Node<'a> {
    let root = Node::new("A", arena);

    let b = Node::new("B", arena);
    let c = Node::new("C", arena);
    let d = Node::new("D", arena);
    let e = Node::new("E", arena);
    let f = Node::new("F", arena);

    unsafe {
        (*root.edges.get()).push(b);
        (*root.edges.get()).push(c);
        (*root.edges.get()).push(d);

        (*c.edges.get()).push(e);
        (*c.edges.get()).push(f);
        (*c.edges.get()).push(root);
    }

    root
}

pub fn main() {
    let arena = TypedArena::new();
    let g = init(&arena);
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    foo(g.first());
}
