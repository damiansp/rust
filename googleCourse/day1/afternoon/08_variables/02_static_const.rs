const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);
static BANNER: &str = "Welcome to Rust Bucket!";


fn main() {
    println!("{BANNER}");
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}


fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (i, &b) in text.as_bytes().iter().enumerate() {
        digest[i % DIGEST_SIZE] = digest[i % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}