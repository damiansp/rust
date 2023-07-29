fn main() {
    let status = 
        if cpu.temp <= MAX_TEMP { HttpStatus::Ok }
        else { HttpStatus::ServerError };
    println!(
        "Inside the vat, you see {}.",
        match vat.contents {
            Some(brain) => brain.descr(),
            None => "nothing of interest"
        });
    
}