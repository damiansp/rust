fn main() {
    let v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3, 4, 5];
    let mut x = Vec::new();
    x.push(5);
    x.push(6);
    x.push(7);

    let third: &i32 = &w[2];
    println!("The third element of w is {}", third);
    let third: Option<&i32> = w.get(2);
    match third {
        Some(third) => println!("The third element of w is {}", third),
        None => println!("There is no third element.")
    }

    // let does_not_exist = &w[100];
    // let does_not_exist = w.get(100);

    let mut u = vec![1, 2, 3, 4, 5];
    let first = &u[0];
    // u.push(6);  // err: attempting to mutate while holding a reference
}