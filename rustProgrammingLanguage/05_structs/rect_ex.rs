fn main() {
    let rect = Rectangle{width: 30, height: 50};
    println!("Area: {}", get_area(&rect));
    println!("rect is {:#?}", rect);
    println!("Area again: {}", rect.area());
}


fn get_area1(width: u32, height: u32) -> u32 {
    width * height
}


fn get_area2(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}


fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}


#[derive(Debug)]  // allows println!() defaults
struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}