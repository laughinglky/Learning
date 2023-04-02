use std::env;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];
}