use colored::*;
use std::io::Write;

fn print_board(board: Vec<Vec<String>>) {
    println!("---------");
    let mut m = 0;
    for array in board {
        for item in array {
            print!("|");
            if item == "null" {
                print!("{}", "-".cyan())
            } else {
                if item == "X" {
                    print!("{}", "X".green())
                } else {
                    print!("{}", "O".red())
                }
            }
            print!("|");
            if m == 2 {
                println!("");
                m = -1
            }
            m += 1;
        }
    }
    println!("---------");
}
fn get_moves() -> (i32, i32) {
    print!("What is the first item?: ");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let item1: i32 = line.trim().parse().unwrap();
    print!("What is the second item?: ");
    std::io::stdout().flush().unwrap();
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).unwrap();
    let item2: i32 = line1.trim().parse().unwrap();
    return (item1, item2);
}
fn new_board() -> Vec<Vec<String>> {
    return vec![
        vec![
            String::from("null"),
            String::from("null"),
            String::from("null"),
        ],
        vec![
            String::from("null"),
            String::from("null"),
            String::from("null"),
        ],
        vec![
            String::from("null"),
            String::from("null"),
            String::from("null"),
        ],
    ];
}
fn main() {
    print_board(new_board());
    get_moves();
}
