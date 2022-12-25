mod unrooted_must_root_lint {
    pub use noop_attr::noop as must_root;
}

/*#[derive(derive::JSTraceable)]
struct S;*/

#[unrooted_must_root_lint::must_root]
struct U {
    _v: i32,
}

trait JSTraceable {
    fn trace(&self) {}
}

fn foo<T: JSTraceable>(t: &T) {
    t.trace();
}

fn main() {
    let _u = U { _v: 0 };
    //let s = S;
    //foo(&s);
    println!("Hello, world!");
}
