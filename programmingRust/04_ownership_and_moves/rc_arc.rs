// arc is thread-safe whereas rc is faster but not thread-safe
use std::rc::Rc;


fn main() {
    // Rust can infer all types here; added just for clariy
    let s: Rc<String> = Rc::new("shiratake".to_string()); // pointer to heap-allocated String (with ref count)
    let t: Rc<String> = s.clone();                        // not a copy, just a new pointer
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert!(t.find("take"), Some(5));
    println!("{} are chewy, almost bouncy, yet lack flavor", u);

    //s.push_str(" noodles") // err: cannot borrow rc data as mutable
}