use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let contents_attempt = File::open("hello.txt");
    let contents = match contents_attempt {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e)
                }
            }
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };
}