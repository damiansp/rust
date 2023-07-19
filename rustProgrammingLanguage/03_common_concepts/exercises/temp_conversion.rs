fn main() {
    for i in 0..=10 {
        let f: f32 = 10f32 * i as f32;
        println!("{f}F = {}C", to_celsius(f));
    }
    for i in 0..=10 {
        let c: f32 = 10f32 * i as f32;
        println!("{c}C = {}F", to_fahrenheit(c));
    }
}


fn to_celsius(f: f32) -> f32{
    (f - 32.) * (5. / 9.)
}


fn to_fahrenheit(c: f32) -> f32 {
    c * (9. / 5.) + 32.
}