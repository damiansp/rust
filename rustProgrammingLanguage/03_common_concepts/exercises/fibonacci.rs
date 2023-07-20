fn main() {
    for i in 0..=20 {
        println!("{i}: {}", fib(i));
    }
}


fn fib(n: i64) -> i64{
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 2) + fib(n - 1)
}