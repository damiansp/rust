use text_colorizer::*;


fn main() {
}


#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}


fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string to another", 
        "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}