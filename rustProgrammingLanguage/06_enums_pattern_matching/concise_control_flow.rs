fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max configured to be {max}"),
        _ => ()
    }
    // alternately
    if let Some(max) = config_max {
        println!("Max configured to be {max}");
    }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1
    }
    // alt
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}