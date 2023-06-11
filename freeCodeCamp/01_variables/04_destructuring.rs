fn main() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
 
    let (z, w);
    (z, ..) = (3, 4);
    [.., w] = [1, 2];
    assert_eq!([z, w], [3, 2]);
    println!("Success");
}