extern crate rand;
use rand ::thread_rng;
use rand :: seq :: SliceRandom;

// struct SudokuNum {
//     value: i8,
//     display: bool
// }

#[derive(Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

struct Sudoku3x3 {
    matrix: [[u8;9];9],
    hint: u8,
}

impl Sudoku3x3 {
    // fn coordinate_convert(x_coordinate: usize, y_coordinate: usize) -> coordinate {
    //     let x = x_coordinate - 1;
    //     let y = 9 - y_coordinate;
    //     coordinate{x, y}
    // }

    fn set_num(&mut self, value: u8,coordinate: Coordinate) {
        self.matrix[coordinate.y][coordinate.x] = value;
    } 

    fn get_num(&self, coordinate: Coordinate) -> u8 {
        self.matrix[coordinate.y][coordinate.x]
    }

    fn print_sudoku(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn same_num_in_row(&self, num:u8, row:usize) -> bool {
        for i in 0..self.matrix.len() {
            if self.matrix[row][i] == num {
                return true
            }
        }
        false
    }

    fn same_num_in_column(&self, num:u8, col:usize) -> bool {
        for i in 0..self.matrix.len() {
            if self.matrix[i][col] == num {
                return true
            }
        }
        false
    }

    fn same_num_in_grid(&self, num:u8, x:usize, y:usize) -> bool {
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

    fn valid_num(&self, num:u8, coordinate: Coordinate) -> bool {
        if self.same_num_in_column(num, coordinate.x) {
            return false
        }
        if self.same_num_in_row(num, coordinate.y) {
            return false
        }
        if self.same_num_in_grid(num, coordinate.x, coordinate.y) {
            return false
        }
        return true
    }

    fn find_empty_square(&self) -> Option<Coordinate> {
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix.len() {
                if self.matrix[row][col] == 0 {
                    let coordinate = Coordinate{x: col, y: row};
                    return Some(coordinate)
                }
            }
        }
        None
    }

    fn generate_full_board(&mut self) -> bool {
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix.len() {
            if self.matrix[row][col] == 0 {
                let list = rng_sudoku();
                for num in list {
                    let num_coordinate = Coordinate{x: col, y: row};
                    if self.valid_num(num, num_coordinate){
                        self.set_num(num, num_coordinate);
                        let not_finished : Option<Coordinate> = self.find_empty_square();
                        match  not_finished {
                            None => return true,
                            Some(coordinate) => if self.generate_full_board() {
                                return true
                            }
                        }
                    }
                break
                }
            }
        }
    }   
    return false    
    }
    
    fn generate_full_board_v2(&mut self) -> bool {
        for row in 0..self.matrix.len() {
            for col in 0.. self.matrix.len(){
                let zeroes : Option<Coordinate> = self.find_empty_square();
                match zeroes {
                    None => return true,
                    Some(coordinate) => {
                        let list = rng_sudoku();
                        if self.matrix[row][col] == 0 {
                            for num in list {
                                let num_coordinate = Coordinate{x: col, y: row};
                                if self.valid_num(num, num_coordinate) {
                                    self.set_num(num, num_coordinate)
                                }
                            }
                        }
                        self.matrix[row][col] = 0;
                    }
                }
            }
        }
        return false
    }
    
}

fn rng_sudoku() -> Vec<u8> {
    let mut x: Vec<u8> = (1..10).collect();
    x.shuffle(&mut thread_rng());
    return x
}

fn main() {

    let mut my_sudoku = Sudoku3x3 {
        matrix: [[0;9];9],
        hint: 3
    };
    my_sudoku.generate_full_board_v2();
    my_sudoku.print_sudoku();

}
