use std::mem::size_of_val


fn main() {
    let _v: () = ();
    let v = (2, 3);
    assert_eq!(v, implicit_unit());

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("Success");
}


fn implicit_unit() {
    println!("I return unit: ()");
}