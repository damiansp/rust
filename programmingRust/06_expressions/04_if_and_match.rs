fn main() {
    let suggest_pet = if with_wings { Pet::Buzzard } else { Pet::Hyena };
    // err: type mismatch
    //let favorite_number = if user.is_hobbit() { "eleventy-one" } else { 9 };
    //let best_team = if is_hockey_season() { "Predators"} // err; no assignment if else
}

match params.get("name") {
    Some(name) => println!("Hello, {}!", name),
    None => println!("Howdy, Stranger.")
}


let score => match card.rand {
    Jack => 10,
    Queen => 11,
    King => 12,
    Ace => 13
};