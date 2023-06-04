fn main() {
    let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    pretty_print(&array);
    println!();
    let t = transpose(array);
    pretty_print(&t);
}


fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for col in row {
            print!("{col} ");
        }
        println!();
    }
}


fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut t = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            t[i][j] = matrix[j][i];
        }
    }
    t
}