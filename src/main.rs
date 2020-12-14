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
    fn coordinate_convert(x_coordinate: usize, y_coordinate: usize) -> coordinate {
        let x = x_coordinate - 1;
        let y = 9 - y_coordinate;
        coordinate{x, y}
    }

    fn set_num(&mut self, x_coordinate: usize, y_coordinate : usize, value: i8) {
        let x = x_coordinate - 1;
        let y = 9- y_coordinate ;
        self.matrix[y][x] = value;
    } 

    fn get_num(&self, x_coordinate: usize, y_coordinate: usize) -> i8 {
        let x = x_coordinate - 1;
        let y = 9- y_coordinate;
        self.matrix[y][x]
    }

    fn print_sudoku(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn same_num_in_row(&self, num:i8, row:usize) -> bool {
        for i in 0..9 {
            if self.matrix[row][i] == num {
                return true
            }
        }
        false
    }

}

fn main() {
    let mut my_sudoku = Sudoku3x3 {
        matrix: [[0;9];9],
        hint: 3
    };
    my_sudoku.set_num(9, 9, 3);
    let a = my_sudoku.get_num(5, 5);
    my_sudoku.print_sudoku();
    println!("{}",a);
}
