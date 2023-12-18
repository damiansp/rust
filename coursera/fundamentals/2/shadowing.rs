fn main() {
    let mut height = 190;
    height = height - 20;
    let res = if height > 180 { "Tall" }
        else if height > 170 { "Avg" }
        else { "Short" };
    println!("Res: {res}");
    let health = if height < 180 { "good" } else { "unknown" };
    println!("Health: {health");
    // type-changing shadowing allowed (with 'let'):
    let health = if height < 180 { true } else { false };
}