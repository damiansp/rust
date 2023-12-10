//--------//--------//--------//--------//--------//--------//--------//--------
use rand::Rng;
use std::io;


fn main() {
    let _ = generate_number(100);
}


struct Player {
    name: String,
    score: u32
}


trait Printable {
    fn to_string(&self) -> String;
}


impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    } 
}


fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => continue
        }
    }
}


fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut n_players = 0;
    loop {
        n_players = collect_input::<u32>("How many plyers (2+)?");
        if n_players < 2 {
            println!("Invalid number. Try again.");
            continue;
        } else {
            break;
        }
    }
    for i in 1..=n_players {
        let name = collect_input(format!("Player {i} name").as_str());
        players.push(Player{name, score: 0});
    }
    players
}


fn create_max_range(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}


fn generate_number(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}