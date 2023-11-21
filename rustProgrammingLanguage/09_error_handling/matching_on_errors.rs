use std::fs::File;


fn main() {
    let contents_attempt = File::open("hello.txt");
    let contents = match contents_attempt {
        Ok(file) => file,
        Err(error) => { panic!("Problem opening the file: {:?}", error); }
    };
}