use colored::*;

fn printBoard(board: Vec<Vec<String>>) {
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
fn newBoard() -> Vec<Vec<String>> {
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
    printBoard(newBoard());
}
