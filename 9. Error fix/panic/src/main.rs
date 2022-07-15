fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let v = vec![1, 2, 3];
    v[99];
}
