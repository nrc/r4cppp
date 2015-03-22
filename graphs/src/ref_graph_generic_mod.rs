
use std::mem;
use std::collections::HashSet;
use arena::TypedArena;

mod graph<'a> {
    struct Node {
        datum: &'static str,
        next: Vec<&'a Node<'a>>,
    }

    impl Node {
        fn new(datum: &'static str, arena: &'a TypedArena<Node>) -> &'a Node {
            arena.alloc(Node {
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

    fn foo(node: &'a Node) {
        println!("foo: {}", node.datum);
    }

    fn init(arena: &'a TypedArena<Node>) -> &'a Node {
        let root = Node::new("A", arena);

        let b = Node::new("B", arena);
        let c = Node::new("C", arena);
        let d = Node::new("D", arena);
        let e = Node::new("E", arena);
        let f = Node::new("F", arena);

        unsafe {
            let mut_root: &mut Node = mem::transmute(root);
            mut_root.next.push(b);
            mut_root.next.push(c);
            mut_root.next.push(d);

            let mut_c: &mut Node = mem::transmute(c);
            mut_c.next.push(e);
            mut_c.next.push(f);
            mut_c.next.push(root);
        }

        root
    }
}

pub fn main() {
    let arena = TypedArena::new();
    let g = graph::init(&arena);
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    foo(g.first());
}
