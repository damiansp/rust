fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let res = largest(&numbers);
    println!("Max is: {res}");

    let numbers = vec![43, 5, 52, 1, 56];
    let res = largest(&numbers);
    println!("Max is: {res}");
}


fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}
