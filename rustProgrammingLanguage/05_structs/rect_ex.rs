fn main() {
    let rect = Rectangle{width: 30, height: 50};
    println!("Area: {}", get_area(&rect));
    println!("rect is {:#?}", rect);
    println!("Area again: {}", rect.area());
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold rect3: {}", rect.can_hold(&rect3));
    let square = Rectangle::square(12);
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
    fn square(size: u32) -> Self {  // Self aliases type after <impl> (here: Rectangle)
        Self {width: size, height: size}    
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}