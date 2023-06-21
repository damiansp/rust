fn main() {
    let f: bool = false;
    let t = true;
    let t2 = true && false;
    assert_eq!(t2, f);
    if t {
        println!("Success");
    }
}