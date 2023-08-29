use crate::garden::vegetables::Asparagus;

pub mod garden;


fn main() {
    let plant = Asparagus {};
    println!("There is {:?} growing in the garden", plant);
}
