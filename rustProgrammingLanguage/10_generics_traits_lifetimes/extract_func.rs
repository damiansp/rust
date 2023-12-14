use std::cmp::PartialOrd;

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let res = largest(&numbers);
    println!("Max is: {res}");

    let chars = vec!['q', 'a', 'y', 'm', 's'];
    let res = largest(&chars);
    println!("Max is: {res}");
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}
