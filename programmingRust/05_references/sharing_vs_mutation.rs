fn main() {
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    let aside = v; // v is moved here
    //r[0]; // error: uses v which is uninitialized
}