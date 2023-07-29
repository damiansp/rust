fn main() {
    let mut s = String::from("Hello");
    change(&mut s);

    let r1 = &mut s;
    //let r2 = &mut s;  // nope: connot have mulitple concurrent mut refs
    //println!("{r1}, {r2}");

    let mut t = String::from("hello");
    {
        let r3 = &mut t;
    } // r1 out of scope
    let r4 = &mut t;  // no problem

    let mut u = String::from("Hello");
    let r5 = &u; // ok
    let r6 = &u; // ok
    //let r7 = &mut v; // NOPE

    let mut v = String::from("howdy");
    let r8 = &v;  // ok
    let r9 = &v;  // ok
    println!("{r8}, and {r9}");  //  r8, r9 no longer used
    let ra = &mut v; // ok
    println!("{ra}");
}


fn change(s: &mut String) { s.push_str(", World!"); }