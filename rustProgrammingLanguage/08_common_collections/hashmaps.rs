use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (k, v) in &scores {
        println("{k} -> {v}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field);  // consumed here

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);  // overwrites
    println!("{:?}", scores2);

    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    let txt = "hello world wonderful world";
    let mut tmap HashMap::new();
    for word in txt.split_whitespace() {
        let count = tmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", tmap);
}