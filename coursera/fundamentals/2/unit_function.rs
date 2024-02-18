fn main() {
    process_numbers(&[1, 2, 3]);
}


fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;
    for n in numbers { sum += n; }
    println!("Sum: {sum}");
    if sum % 2 == 0 { println!("Sum is even"); }
    else { println!("Sum is odd"); }
}