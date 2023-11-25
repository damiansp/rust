use std::fs::File;


fn main() {
    let contents = File::open("hello.txt").unwrap();
    let take2 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");  // <- err msg on fail
}