//don't need mod declaration here because alr declared in main.rs
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
const BOARD_SIZE: usize = 9;

pub fn print_hello() {
    println!("Hello, world!");
}

#[derive(Debug)] //so we can print out Sudoku for debugging with println!({:?}, Sudoku);
//can also use {:#?}, which is pretty printing of Sudoku
pub struct Sudoku {
    board: Vec<Vec<i32>>, //board is a vec of vec
    empty_cells: Vec<(usize,usize)>, //hold pair of row,col; so need usize as type
}

//constructor, kind of
impl Sudoku {
    //constructor, return a vector of vectors with capacity = 9
    pub fn new() -> Sudoku {
        let vec: Vec<Vec<i32>> = Vec::with_capacity(BOARD_SIZE); //empty vector
        let vec2: Vec<(usize,usize)> = Vec::new(); //empty vec
        Sudoku { 
            board : vec, 
            empty_cells : vec2,
        }
    }
    
    pub fn print(&self) {
        // println!("printing board");
        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell != 0 { //cell is borrowed, and it's like a pointer
                    print!("{} ", cell);
                } else {
                    print!("  ");                    
                }
                if j == 2 || j == 5 {
                    print!("| ");
                }
            }
            println!();
            if i == 2 || i == 5 {
                println!("------+-------+------");
            }
        }
    }

    pub fn solve(&mut self) -> bool {
        if self.solve_empty_cell(0) {
            self.empty_cells.clear();
            return true;
        } else {
            self.empty_cells.clear();
            return false;
        }
    }

    fn solve_empty_cell(&mut self, num: usize) -> bool {
        if num >= self.empty_cells.len() {
            return true;
        }
        let row = self.empty_cells[num].0;
        let col = self.empty_cells[num].1;
        for val in 1..10 {
            if self.is_valid_move(val, row, col) {
                self.set_cell(val, row, col);
                if self.solve_empty_cell(num + 1) {
                    return true;
                }
                self.unset_cell(row, col);
            }
        }        
        return false;
    }

    //load sudoku puzzle's board from file
    pub fn load_from_file(&mut self, filename : &str) {
        let file = match File::open(filename.to_string()) {
            Err(why) => panic!("couldn't open {}: {}", filename,
                                                   why.description()),
            Ok(file) => file,
        };
        
        let mut reader = BufReader::new(file);

        for line in reader.by_ref().lines() {
            let numbers: Vec<i32> = line
                .unwrap() //remove the extra stuff around our string
                .split_whitespace() //split using " " as delimiter
                .map(|x| x.parse().expect("Not an integer!")) //parse the strings as numbers
                .collect(); //collect into a vec
            self.board.push(numbers);
        }

        //load empty_cells, aka mark positions to be filled
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col] == 0 {
                    self.empty_cells.push((row,col));
                }
            }
        }
    }

    //row and col has to be usize because usize is pointer-sized for all architectures
    fn set_cell(&mut self, val: i32, row: usize, col: usize) {
        self.board[row][col] = val;
    }

    fn unset_cell(&mut self, row: usize, col: usize) {
        self.board[row][col] = 0;
    }

    fn is_valid_move(&mut self, val: i32, row: usize, col: usize) -> bool {
        for i in 0..BOARD_SIZE {
            if self.board[row][i] == val {
                return false;
            }
        }

        for j in 0..BOARD_SIZE {
            if self.board[j][col] == val {
                return false;
            }
        }

        //check within box
        let box_row = row/3;
        let box_col = col/3;
        
        for y in 3 * box_row .. 3 * box_row + 3 {
            for x in 3 * box_col .. 3 * box_col + 3 {
                if self.board[y][x] == val {
                    return false;
                }
            }
        }
        return true;
    }
}
