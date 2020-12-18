
extern crate rand;
use rand :: Rng;
use rand ::thread_rng;
use rand :: seq :: SliceRandom;
use std :: io;

// struct SudokuNum {
//     value: i8,
//     display: bool
// }

#[derive(Copy, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize
}

pub struct CellProperties {
    val: u8,
    coordinate: Coordinate
}
#[derive(Default)]
pub struct Sudoku3x3 {
    pub matrix: [[u8;9];9],
    pub solved_matrix: [[u8;9];9],
    pub hint: u8,
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
    
    fn get_num_solved(&self, coordinate: Coordinate) -> u8 {
        self.solved_matrix[coordinate.y][coordinate.x]
    }

    fn copy_completed_matrix(&mut self) {
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix.len() {
                self.solved_matrix[row][col] = self.matrix[row][col];
            }
        }
    }
    pub fn print_sudoku(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }
    // should panic if solved_matrix is not truly sudoku puzzle 
    pub fn print_completed_sudoku(&self) {
        for row in &self.solved_matrix {
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
    // use backtracking recursion algorithm, find the first possible sudoku board 
    pub fn generate_full_board(&mut self) -> bool {
        let zeroes = self.find_empty_square();
        match zeroes {
            None => {
                self.copy_completed_matrix();
                return true
            }
            Some(coordinate) => {
                let numlist = rng_sudoku();
                for num in numlist {
                    if self.valid_num(num, coordinate) {
                        self.set_num(num, coordinate);
                        let done = self.generate_full_board();
                        if done {
                            return true
                        }
                        self.set_num(0, coordinate);
                    }
                }
            }
        }
        return false
    }
    // Algorithm to remove 1 random number from the sudoku.
    // If the number is already removed
    // return None
    fn remove_random_num(&mut self) -> Option<CellProperties> {
        let mut rng = rand::thread_rng();
        let row: usize = rng.gen_range(0,9);
        let col: usize = rng.gen_range(0,9);
        let num: u8 = self.matrix[row][col];
        let coordinate = Coordinate{x: col, y: row};
        if self.matrix[row][col] != 0 {
            self.set_num(0, coordinate);
            let  removed_cell = CellProperties {val: num, coordinate: coordinate};
            return Some(removed_cell)
        }
    return None
    }
    // Algorithm to remove number from full board to a puzzle
    // If sudoku has more than 1 solution, automatically stop 
    // and add the last number back to the position.
    pub fn remove_some_num(&mut self, zero_count: u8) -> bool {
        let mut count = 0;
        let mut removed_success : Vec<CellProperties> = Vec::new();
        while count < zero_count {
            let m = self.remove_random_num();
            match m {
                None => (),
                Some(m) => { 
                    removed_success.push(m);
                    count += 1;
                }
            }
            if self.solution_count() > 1 {
                let last_cell_modified = removed_success.pop();
                match last_cell_modified {
                    None => (),
                    Some (last_cell_modified) => {
                        self.set_num(last_cell_modified.val, last_cell_modified.coordinate);
                        return true
                    }
                }                    
            }
        }
        return true
    }

    // counting solution function. 
    // If there's more than 1 solution, the puzzle is not proper sudoku
    pub fn solution_count(&mut self) -> i32 {
        let mut count: i32 = 0;
        let zeroes = self.find_empty_square();
        match zeroes {
            None => return 1,
            Some(coordinate) => {
                let numlist: Vec<u8> = (1..10).collect();
                for num in numlist.iter() {
                    if self.valid_num(*num, coordinate) {
                        self.set_num(*num, coordinate);
                        let done = self.solution_count();
                        count += done;
                        self.set_num(0, coordinate);
                    }
                }
            }
        }
        return count
    }

    fn use_hint(&mut self, coordinate:Coordinate) -> Option<u8> {
        if self.get_num(coordinate) != 0 {
            return None
        }
        else {
            self.set_num(self.get_num_solved(coordinate), coordinate);
            return Some(self.get_num_solved(coordinate))
        }
    }
    pub fn ask_for_hint(&mut self)  {
        if self.hint == 0 {
            println!("Sorry you have no more hints lefts");
        } else {
            let mut row = String::new();
            let mut col = String::new();
            loop {
            println!("please select the location you want to reveal");
            
            println!("Enter row: ");
            io::stdin()
                .read_line(&mut row)
                .expect("Failed to read row value");
            
            let row: usize = match row.trim().parse() {
                Ok(row) => row,
                Err(_) => {println!("please input a number between 1 to 9");
                            break},
            };
            
            println!("Enter column: ");
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read column value");
            
            let col: usize = match col.trim().parse(){
                Ok(col) => col,
                Err(_) => {println!("please input a number between 1 to 9");
                        break},
            };
            if col > 9 || col < 1 || row > 9 || row < 1 {
                println!("column and row value must be between 1 to 9");
                break;
            }  
            let read_coor = Coordinate{y: row -1 , x: col- 1};
            let action = self.use_hint(read_coor);
            match action {
                None => {println!("this is not a blank position");
                        self.print_sudoku();
                    }
                Some(x) => {
                    println!("The number at the position selected is: {}", x);
                    self.hint -=1;
                    println!("Hints left: {}", self.hint);
                    break;
                }
            }
        }
    }
    }
}

fn rng_sudoku() -> Vec<u8> {
    let mut x: Vec<u8> = (1..10).collect();
    x.shuffle(&mut thread_rng());
    return x
}
