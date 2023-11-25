use std::fs::File;


fn main() {
    let contents = File::open("hello.txt").unwrap();
}