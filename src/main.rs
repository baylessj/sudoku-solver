//
// Sudoku Solver
//

// Contains the data for a cell in the puzzle
struct Cell {
    val: i32,
    possibilities: Vec<i32>,
    row: i32,
    col: i32
}

fn init_board() {
    let board:Vec<Vec<i32>> = vec![vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
                                 vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
                                 vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                                 vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                                 vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                                 vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
                                 vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                                 vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                                 vec![0, 0, 0, 0, 8, 0, 0, 7, 9]];
    for (row, i) in board.iter().enumerate() {
        for (col, val) in i.iter().enumerate() {
            println!("row: {} col: {}, val: {}", row, col, val);
        }
    }

    // 2D vector is ez but not performant
    // let working_copy: Vec<Vec<Cell>> = board
    //    .iter()
    //    .map(|x| Cell { val: x, possibilities: vec![1...9], row: 0, col: 0);
}

fn main() {
    println!("Hello, world!");
    init_board();
}

