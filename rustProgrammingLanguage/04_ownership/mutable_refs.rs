fn main() {
    let mut s = String::from("Hello");
    change(&mut s);

    let r1 = &mut s;
    //let r2 = &mut s;  // nope: connot have mulitple concurrent mut refs
    //println!("{r1}, {r2}");
}


fn change(s: &mut String) { s.push_str(", World!"); }