fn main() {
    let x = 10;
    let r = &x;           // shared ref to x
    assert!(*r == 10);  // r explicitly dereferenced

    let mut y = 32;
    let m = &mut y;  // mutable ref to y
    *m += 32;        // explicit deref
    assert!(y == 64);

    let aria = Anime{name: "Aria: The Animation", bechdel_pass: true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // Equivalently (but unnecessarily verbosely):
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();  // implicitly borrowed mut ref; equivalent to
    (&mut v).sort();

    let a = 10;
    let b = 20;
    let mut c = &a;
    if some_condition {
        c = &b;
    }
    assert!(*c == 10 || *c == 20);
}


struct Anime {
    name: &'static str,
    bechdel_pass: bool
}