#[derive(derive::JSTraceable)]
struct S;

#[cfg_attr(dylint_lib = "lint", unrooted_must_root_lint::must_root)]
struct U;

trait JSTraceable {
    fn trace(&self) {}
}

fn foo<T: JSTraceable>(t: &T) {
    t.trace();
}

fn main() {
    let _u = U;
    let s = S;
    foo(&s);
    println!("Hello, world!");
}
