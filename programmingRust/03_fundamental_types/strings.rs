let speech = "\"Ouch!\", said the well.\n";
println(
    "In the room othe women come and go,  
    Singing of Mount Abora.");  // ok but \n exists at line break
println!(
    "It was a bright cold day in April, and \
    there were four of us-\"
    more or less.");  // continuous string

let default_win_install_path = r"C:\Program Files\Gorilla\";
let pattern = Regex::new(r"\d+(\.\d+)*");
println(
    r###"
    This raw string is delimited by 'r###"' at the beginning
    and does not terminate until the matching quote and same
     number of #-signs at the end.
    "###);


let method == b"GET";
assert_eq!(method, &[b'G', b'E', b'T']);

let noodles = "noodles".to_string();  // type String (includes pointer to first char, capacity, length)
let oodles = &noodles[1..];           // type &str   (pointer to first char, length)
let poodles = "ಠ_ಠ";                  // type &str
// type `str` is an unsized type, meaning that it must always be used behind a pointer like &str or Box<str> 
// and holds the actual character values (or their UTF-8 encoding).

// NOTE: String.len() and &str.len() are measured in bytes, not characters.
assert_eq!("ಠ_ಠ".len(), 7);            // 7 bytes
assert_eq!("ಠ_ಠ".chars().count(), 3);  // 3 chars

let mut s = "hello";
//s[0] = 'H';    // error: `&str` cannot be modified
//s.push('\n');  // error: `&str` does not have a method `push`

let err_msg = "too many pets".to_string();
let bits = vec!["veni", "vidi", "vici"];
assert_eq!(bits.concat(), "venividivici");
assert_eq!(bits.join(", "), "veni, vidi, vici");

