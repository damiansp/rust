//3458790//3458790//3458790//3458790//3458790//3458790//3458790//3458790
use std::io;


const THREE_HRS_IN_SECS: u32 = 3 * 60 * 60;


fn main() {
    let mut x = 5;
    println!("x = {x}");
    let x = x + 1;  // no longer mut
    println!("x = {x}");
    {
        let x = 2 * x;
        println!("x (inner) = {x}");
    }
    println!("x = {x}");

    let x = 2.; // f64
    let y: f32 = 3.;

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let trunc = -5 / -3; // -1
    let rem = 43 % 5;
    
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("b: {b}");
    let five_o = tup.0;
    println!("five-o: {five_o}");

    let d = [1, 2, 3, 4, 5];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", 
        "Oct", "Nov", "Dec"];
    let e: [i32; 5] = [1, 2, 3, 4, 5];
    let f = [3; 5]; // [3 3 3 3 3]
    let first = e[0];
    let second = e[1];

    println!("Enter an array index: ");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = 
        index.trim().parse().expect("Index not a number");
    let element = d[index];
    println!(
        "The value of the element at index {} is {:?}", index, element);

    another_fn(42);
    print_labeled_measurement(5, 'h');

    let five = five();
    println!("five: {five}");

    let six = increment(5);
    println!("six: {six}");

    let num = 3;
    if num < 5 {
        println!("< 5");
    } else if num < 10{
        println!("< 10");
    } else {
        println!(">= 10");
    }
    let condition = true;
    let conditional_n = if condition { 5 } else { 6 };
    println!("conditional_n: {conditional_n}");

    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("res: {res}");  // 20

    labeled_loop_ex();
    while_ex();
    for_ex();
    for_in_ex();
}


fn another_fn(x: i32) {
    println!("Another function was passed {x}!");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("{value}({unit_label})");
}


fn five() -> i32 {
    5
}


fn increment(x: i32) -> i32 {
    x + 1
}


fn labeled_loop_ex() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


fn while_ex() {
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("Liftoff!!");
}


fn for_ex() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("a[{index}] = {}", a[index]);
        index += 1;
    }
}


fn for_in_ex() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element = {}", element);
    }
    for n in (1..4).rev() {
        println!("{n}!")
    }
    println!("Blastoff!!!")
}