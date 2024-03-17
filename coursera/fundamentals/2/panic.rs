fn main() {
    loop_and_panic(vec![1, 2, 3, 4, -5]);
}

fn loop_and_panic(ns: Vec<i32>) {
    for n in ns {
        if n < 0 { panic!("Negatives not allowed") };
        println!("Number: {n}");    
    }
}

