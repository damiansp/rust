fn main() {
    game.black_pawns  // struct field
    coord.1           // tuple element
    pieces[i]         // array element
    let second_half = &game_moves[midpoint .. end] // [midpoint, end); 
}


fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 { return; }
    let pivot_index = partition(slice);
    quicksort(&mut slice[.. pivot_index]);
    // why isn't first idx pivot_index?  (err?)
    quicksort(&mut slice[pivot_index + 1 ..]); 
}