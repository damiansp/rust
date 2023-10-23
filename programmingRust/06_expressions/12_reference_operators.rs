fn main() {
    let padovan: Vec<u64> = compute_padovan_sequence(n);
    for elem in &padovan {
        draw_triangle(turtle, *elem);
    }
}