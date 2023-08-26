fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player()  // could also be just: _ => () 
    }


}


fn add_fancy_hat() {
    println!("Your character dons a fancy hat");
}


fn remove_fancy_hat() {
    println!("A strong wind has blown away your fancy hat");
}


fn move_player() {
    let rand = 9;
    println!("Character advances {rand} spaces");
}