fn main() {
    let string1 = "somnambulance".to_string();
    let string2 = string1;  // points to same buffer on heap
    let n1: i32 = 36;
    let n2 = n1;            // actually a copy for ints and other basic types (ints, floats, chars, bools)

    let my_lab = Label{number: 3};
    print(my_lab);
    //println!("My lab is {}", my_lab.number); // compile err; main no longer owns my_lab

    let your_lab = CopyLabel{number: 4};
    print_copy(your_lab);
    println!("Your lab is {}", your_lab.number); // ok
}


fn print(lab: Label) {
    println!("ID #{}", lab.number);
}

struct Label {
    number: u32
}


fn print_copy(lab: CopyLabel) {
    println!("ID #{}", lab.number);
}


#[derive(Copy, Clone)] // passes copy by default
struct CopyLabel {
    number: u32        // all attributes must be copy types
}