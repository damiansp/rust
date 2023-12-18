fn main() {
    let msg = "Name: Damian, Weight: ";
    let weight = 170.;
    const LBS_PER_KG: f64 = 2.2;
    let weight_kg = weight / LBS_PER_KG;
    println!("{}{}", msg, weight_kg);
}