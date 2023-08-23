// Basic patterns:
// while condition { ... }
// while let pattern = expr { ... }
// loop { ... }
// for pattern in iterable { ... }
fn main() {
    for i in 0..20 {
        println!("{i}");
    }

    let strings: Vec<String> = error_messages();
    for s in strings {
        println!("{s}");
    }
    //println!("{} error(s)", strings.len());  // err: strings consumed

    for rs in &strings {
        println!("String {:?} is at address {:p}.", *rs, rs);
    }
    for rs in &mut strings {
        rs.push('\n'); // add newline to each
    }
}