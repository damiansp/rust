use std::mem::size_of_val;


fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    let s1 = "中";
    print_str(&s1);
    println!("Success");
}


fn print_str(s: &str) {
    println!("{s}");
}