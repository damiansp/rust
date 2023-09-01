fn main() {
    let output = File::create(filename)?;  // equiv to:
    let out = match File::create(filename) {
        Ok(f) => f,
        Err(err) => return Err(err)
    };
}


fn nothing() {
    return;  // defaults to ()
}