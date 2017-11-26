pub mod sudoku; //to import this mod from sudoku.rs
extern crate time;
use std::io;
use time::PreciseTime;

fn main() {
    println!("Sudoku Solver");
    println!("-------------"); 
    loop {
        let mut puzzle = sudoku::Sudoku::new();    
        let mut inputfile = String::new();
        println!("Enter name of file containing the Sudoku problem: ");
        io::stdin().read_line(&mut inputfile)
            .expect("Failed to read line");

        puzzle.load_from_file(inputfile.trim());

        println!("\nGiven Puzzle:");
        puzzle.print();

        println!("\nSolved Puzzle:");
        let start = PreciseTime::now();
        if puzzle.solve() {
            puzzle.print(); 
        } else {
            println!("No Solution");
        }
        let end = PreciseTime::now();

        println!("\n{} seconds to solve sudoku.", start.to(end));

        let mut answer = String::new();
        println!("Do you want to run again?\nPress any key to quit, type \"yes\" to continue");
        io::stdin().read_line(&mut answer)
            .expect("Failed to read line");

        let answer = answer.trim();//trim away newline character
        if answer == "yes" || answer == "y" {
            continue;
        } else {
            break;
        }
    }
}
