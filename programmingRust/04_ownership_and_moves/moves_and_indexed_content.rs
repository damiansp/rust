fn main() {
    // build vec wtih strings "101", "102", ..., "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
    //let third = v[2];  // err: cannot move out of index; Alternatives:
    let fifth = v.pop().expect("Vector is empty");
    let second = v.swap_remove(1);  // moves last element into its place
    let third = std::mem::replace(&mut v[2], "sub".to_string());  // remove and put "sub" in its place

    let slogans = vec!["liberté".to_string(), "égalité".to_string(), "fraternité".to_string()];
    for mut s in slogans {
        s.push('!');
        println!("{s}");
    }

    let mut composers = Vec::new();
    composers.push(Person{name: Some("Palestringa".to_string()), birth_year: 1525});

    // Nope:
    //let first_name = composers[0].name; // cannot move out of index
    let first_name = std::mem::replace(&mut composers[0].name, None);  // or, equivalently:
    let first_name = composers[0].name.take();
}


struct Person {
    name: Option<String>, 
    birth_year: i32
}