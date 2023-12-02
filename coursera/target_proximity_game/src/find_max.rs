fn main() {
    let l1 = [1, 5, 8, 2];
    let max = find_max(&l1).unwrap().to_owned();
    println!("Max: {max}");

    let l2: [_; 0] = [];
    let max2 = find_max::<Option<&u32>>(&l2);
    println!("Max: {:?}", max2);
}


fn find_max<T: PartialOrd>(v: &[T]) -> Option<&T> {
    if v.is_empty() {
        return None;
    }
    let mut max = &v[0];
    for item in v.iter() {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

