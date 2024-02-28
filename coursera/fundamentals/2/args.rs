fn main() {
    let ns = [1, 2, 3, 4, 5];
    let res = sum(&ns);
    println!("Sum is {res}");
}


fn sum(ns: &[i32])  -> i32 {
    let mut res = 0;
    for n in ns { res += n; }
    res
}