fn main() {
    let x = 1_000.000_1;
    let y: f32 = 0.12; 
    let z = 0.01_f64;
    assert_eq!(type_of(&x), "f64".to_string());

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
    println!("Success");
}


fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}