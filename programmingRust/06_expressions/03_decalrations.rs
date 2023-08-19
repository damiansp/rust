use std::io;
use std::cmp::Ordering;


fn main() {
    let name;
    if user.has_nickname() { 
        name = user.nickname();
    } else {
        name = generate_unique_name();
        user.register(&name);
    }
}


fn show_files() -> io::Result<()> {
    let mut v = vec![];
    // ...

    fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp).reverse().then(a.path.cmp(&b.path))
    }

    v.sort_by(cmp_by_timestamp_then_name);
}