use std::time::Instant;

enum Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut var_cells = vec![];
        for i in 0 .. 9 {
            for j in 0 .. 9 {
                if board[i][j] == '.' {
                    var_cells.push((i, j))
                }
            }
        }

        let mut i = 0;

        while i < var_cells.len() {
            let (x, y) = var_cells[i];

            let current = board[x][y];

            if current == '.' {
                board[x][y] = '1'
            }
            else if current == '9' {
                board[x][y] = '.';
                i -= 1;
                continue
            }
            else {
                board[x][y] = std::char::from_u32(current as u32 + 1).expect(&format!("{}", board[x][y]))
            };

            if Solution::is_valid(board, x, y) {
                i+= 1;
            }
        }

    }

    fn is_valid(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let new_item = board[x][y];

        for i in 0 .. 9 {
            if board[x][i] == new_item && i != y {
                return false
            }
        }

        for i in 0 .. 9 {
            if board[i][y] == new_item && i != x {
                return false
            }
        }

        let x_start = (x/3)*3;
        let y_start = (y/3)*3;
        for i in x_start .. x_start + 3  {
            for j in y_start .. y_start + 3 {
                if board[i][j] == new_item && !(i == x && j == y) {
                    return false
                }
            }
        }

        true
    }

}

#[test]
fn test() {
    let mut data: Vec<Vec<_>> = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];

    let start = Instant::now();
    Solution::solve_sudoku(&mut data);
    let duration = start.elapsed();

    println!("Took {} s", duration.as_secs_f64());

    for i in 0 .. 9 {
        for j in 0 .. 9 {
            print!("{} ", data[i][j]);

            if j == 2 || j == 5 {
                print!("| ")
            }
        }
        if i == 2 || i == 5 {
            print!("\n------+------+-------");
        }
        println!();
    }

}
