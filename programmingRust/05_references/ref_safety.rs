fn main() {
    // Borrowing a local var
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        //assert_eq!(*r, 1); // nope, x no longer in scope
    }   
}