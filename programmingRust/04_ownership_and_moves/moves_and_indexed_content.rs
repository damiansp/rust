fn main() {
    // build vec wtih strings "101", "102", ..., "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
    let third = v[2];  // err: cannot move out of index
}