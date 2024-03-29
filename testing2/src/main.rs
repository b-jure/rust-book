use testing2::solver::{fibonnaci, self};

fn main() {
    let input = 5;
    fibonnaci(input);

    let easy_sudoku = [
        [9,8,4,2,7,0,0,3,1],
        [6,1,3,9,4,5,0,2,0],
        [2,5,7,1,3,8,0,0,9],
        [8,3,2,7,5,0,4,9,0],
        [0,4,0,0,9,0,0,1,8],
        [0,0,6,0,8,2,0,0,3],
        [3,7,8,0,1,0,9,0,0],
        [4,0,0,0,0,7,0,0,0],
        [5,6,0,0,0,0,0,0,4]
    ];

    let board = solver::solve(easy_sudoku);
    for row in board.iter() {
        println!("{:?}", row);
    }

}
