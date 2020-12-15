// struct SudokuNum {
//     value: i8,
//     display: bool
// }
// #[derive(Copy, Clone)]
struct coordinate {
    x: usize,
    y: usize
}

struct Sudoku3x3 {
    matrix: [[i8;9];9],
    hint: i8,
}

impl Sudoku3x3 {
    // fn coordinate_convert(x_coordinate: usize, y_coordinate: usize) -> coordinate {
    //     let x = x_coordinate - 1;
    //     let y = 9 - y_coordinate;
    //     coordinate{x, y}
    // }

    fn set_num(&mut self, value: i8, x: usize, y : usize) {
        self.matrix[y][x] = value;
    } 

    fn get_num(&self, x: usize, y: usize) -> i8 {
        self.matrix[y][x]
    }

    fn print_sudoku(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn same_num_in_row(&self, num:i8, row:usize) -> bool {
        for i in 0..self.matrix.len() {
            if self.matrix[row][i] == num {
                return true
            }
        }
        false
    }

    fn same_num_in_column(&self, num:i8, col:usize) -> bool {
        for i in 0..self.matrix.len() {
            if self.matrix[i][col] == num {
                return true
            }
        }
        false
    }

    fn same_num_in_grid(&self, num:i8, x:usize, y:usize) -> bool {
        let sub_row = y / 3 * 3;
        let sub_col = x / 3 * 3;
        for i in sub_row..sub_row + 3 {
            for j in sub_col..sub_col + 3 {
                if self.matrix[i][j] == num {
                    return true
                }
            }
        }
        false
    }
    fn valid_num(&self, num:i8, x:usize, y:usize) -> bool {
        if self.same_num_in_column(num, x) {
            return false
        }
        if self.same_num_in_row(num, y) {
            return false
        }
        if self.same_num_in_grid(num, x, y) {
            return false
        }
        return true
    }
}

fn main() {
    let mut my_sudoku = Sudoku3x3 {
        matrix: [[0;9];9],
        hint: 3
    };
    my_sudoku.set_num(3, 8, 8);
    my_sudoku.print_sudoku();
    println!("{}", my_sudoku.same_num_in_column(3,8));
    println!("{}", my_sudoku.same_num_in_row(3,7));
    println!("{}", my_sudoku.same_num_in_grid(3,7,6));
}
