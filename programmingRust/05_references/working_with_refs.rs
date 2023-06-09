static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;


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
    //if some_condition {
    //    c = &b;
    //}
    //assert!(*c == 10 || *c == 20);

    let point = Point{x: 1000, y: 729};
    let pp: &Point = &point;
    let ppp: &&Point = &pp;
    let pppp: &&&Point = &ppp;  // but why would you do this??!!
    assert_eq!(pppp.y, 729);

    let d = 10;
    let e = 10;
    let rd = &d;
    let re = &e;
    let rrd = &rd;
    let rre = &re;
    assert!(rrd == rre);             // values equal
    assert!(!std::ptr::eq(rd, re));  // not same objects in memory
    //assert!(rd == rrd);            // err type mismatch: &i32, &&i32
    assert!(rd == *rrd);             // ok

    let f = &factorial(6);
    assert_eq!(f + &1009, 1729);

    // Receiving refs as func args
    f(&WORTH_POINTING_AT);
    println!("STASH: {STASH}");

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
    }
    // assert_eq!(*s, 0); // points to elem of dropped array

    {
        let parab = [9, 4, 1, 0, 1, 4, 9];
        let t = smallest(&parab);
        assert_eq!(*t, 0); // ok, parab still alive
    }

    let u;
    {
        let x = 10;
        u = S{ r: &x };
    }
    // reads from dropped x; won't compile w/o giving r static life in S
    //assert_eq!(*s.r, 10); 

    let v = 10;
    let q;
    {
        let p = 20;
        {
            let t = T{x: &v, y: &p};
            q = t.x;
        }
    }
    println!("{r}")
}


struct Anime {
    name: &'static str,
    bechdel_pass: bool
}


struct Point {
    x: i32,
    y: i32
}


fn factorial(n: usize) -> usize {
    (1 .. n + 1).product()
}


fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}


// fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}


struct S {
    r: &'static i32
}

// alternately (and better, I'd say)
srtuct S2<'a> {
    r: &'a i32
}


struct TBad {
    x: &'a i32,
    y: &'a i32
}


struct T<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}