mod lib;
use std::io;
use std::thread;
use std::time;

fn main() {
    let wait_short = time::Duration::from_millis(800);
    let wait_long = time::Duration::from_millis(1300);
    println!("Generating sudoku....");
    thread::sleep(wait_long);
    let mut my_sudoku = lib::Sudoku3x3 {
        matrix: [[0;9];9],
        solved_matrix: [[0;9];9],
        hint: 3
    };
    my_sudoku.generate_full_board();
    my_sudoku.remove_some_num(50);
    my_sudoku.print_sudoku();   
    
    
    loop {
        println!("Option: 1 for hint, 2 for complete solution");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to get choice");

        let choice: u8 = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {println!("only accept number as choice");
                        continue}
        };
        println!("you choose: {} \n", choice);
        match choice {
            1 =>    { my_sudoku.ask_for_hint();
                    thread::sleep(wait_long);
                    my_sudoku.print_sudoku();
                    thread::sleep(wait_short);
                }
            2 =>    { println!("Getting completed solution...");
                    thread::sleep(wait_long);
                    my_sudoku.print_completed_sudoku();
                    thread::sleep(wait_short);
                    println!("Game over!");
                    thread::sleep(wait_short); 
                    println!("Thanks for playing");
                    thread::sleep(wait_short);
                    break
                }
            _ => println!("only 1 and 2 can be chosen"),
            }
        }
    }
