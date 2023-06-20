fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1;  // string on stack has been "moved" (rebound to s2); s1 no longer accessible
}