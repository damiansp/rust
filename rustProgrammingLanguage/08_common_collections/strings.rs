fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();  // or let s = "initial contents".to_string();
    let s2 = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let mut f = String::from("foo");
    f.push_str("bar");
    let mut f2 = String::from("lo");
    f2.push('l');
    let h = String::from("Hello, ");
    let w = String::from("world!");
    let hw = h + &w; // h has been moved here and can no longer be used
    let t = String::from("tic");
    let u = String::from("tac");
    let v = String::from("toe");
    let ttt = format!("{t}-{u}-{v}");

    //let h_char = h[0];  // err (String is not indexable)

    let ruskij = "Здрастбуйте";
    let zd = &ruskij[0..4];    // not recommended but allowed
    //let huh = &ruskij[0..1]  // not even allowed

    for c in "Зд".chars() {
        println!("{c}");  // З,  д, ...
    }

    for b in "Зд".bytes() {
        println!("{b}");  // 208, 151, ...
    }
}