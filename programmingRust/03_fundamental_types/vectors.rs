let mut primes = vec![2, 3, 5, 7];
primes.push(11);
primes.push(13);


fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols * 3];
}


let mut pal = Vec::new();
pal.push("step");
pal.push("on");
pal.push("no"); 
pal.push("pets");

let v: Vec<i32> = (0..5).collect();

let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
palindrome.reverse();

let mut w = Vec::with_capacity(2);
w.push(1);
w.push(2);
w.push(3);  // ok, just has to allocate more memory

let mut x = vec![10, 20, 30, 40, 50];
x.insert(3, 35);  // insert at index 3
x.remove(1);      // remove the element at index 1

let mut y = vec!["Snow Puff", "Glass Gem"];
assert_eq!(y.pop(), Some("Glass Gem"));
assert_eq!(y.pop(), Some("Snow Puff"));
assert_eq!(y.pop(), None);

let languages: Vec<String> = std::env::args().skip(1).collect();
for lng in languages {
    println!("{}: {}", lng, if lng.len() % 2 == 0 { "functional" } else { "imperative" });
}
