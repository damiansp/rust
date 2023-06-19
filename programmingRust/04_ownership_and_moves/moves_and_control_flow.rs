fn main() {
    let x = vec![10, 20, 30];
    if c {
        f(x); // ok  
    } else {
        g(x); // ok
    }
    h(x)  // bad; x uninitialized if either if/else path used it

    let y = vec![10, 20, 30];
    while f() {
        g(x);  // bad: moved in first iter, uninitialize in 2nd
    }

    let mut z = vec![10, 20, 30];
    while f() {
        g(x);
        x = h()n
    }
    e(x);  // ok
}