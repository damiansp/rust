pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add_two(a: i32) -> i32 { a + 2 }


pub fn greet(name: &str) -> String {
    format!("Hello {name}")
}


pub struct Guess {
    value: i32
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be on [1, 100], got {value}");
        }
        Guess{value}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //#[test]
    //fn another() {
    //    panic!("Make this test fail");
    //}

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width: 5, height: 3};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width: 5, height: 3};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() { assert_eq!(add_two(2), 4); }

    #[test]
    fn greeting_contains_name() {
        let res = greet("Carol");
        assert!(res.contains("Carol"), "Greeting did not contain name: {res}");
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}


