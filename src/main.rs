use colored::*;
use std::io::Write;

fn main() {
    loop {
        print_board(new_board());
        get_moves();
    }
}

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
    let item1: i32 = integer_input("What is the first item?: ");
    let item2: i32 = integer_input("What is the second item?: ");
    return (item1, item2);
}

fn integer_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse().unwrap();
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
#[allow(dead_code)]
fn main_menu() -> Vec<String> {
    return vec![];
}
#[allow(dead_code)]
fn get_player_types() -> Vec<String> {
    return vec![];
}
#[allow(dead_code)]
fn lazy_cpu(board: Vec<Vec<String>>) -> Vec<Vec<String>> {
    return vec![];
}
#[allow(dead_code)]
fn random_cpu(board: Vec<Vec<String>>) -> Vec<Vec<String>> {
    return vec![];
}
fn is_draw(board: Vec<Vec<String>>) -> bool {
    let mut count = 0;
    for array in board {
        for item in array {
            if item == "X" || item == "O" {
                count += 1;
            } else {
                continue;
            }
        }
    }
    if count == 9 {
        return true;
    }
    return false;
}
#[allow(dead_code)]
fn make_move(board: Vec<Vec<String>>, player_icon: &str) -> Vec<Vec<String>> {
    return vec![];
}
