#![feature(rustc_private)]

extern crate arena;

mod rc_graph;
mod rc_refcell_graph;
mod ref_graph;

fn main() {
    println!("Rc<T>:");
    rc_graph::main();
    println!("\nRc<RefCell<T>>:");
    rc_refcell_graph::main();
    println!("\n&T:");
    ref_graph::main();
}
