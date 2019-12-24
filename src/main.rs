use colored::*;
use rand::Rng;
use std::io::Write;

fn main() {
    let mut board = new_board();
    main_menu();
    loop {
        print_board(board.clone());
        board = make_move(board.clone(), String::from("X"));
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
fn prompt_input(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return String::from(line.trim().trim());
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
    let player_types = get_player_types();
    let mut levels = player_types.clone();
    levels.push("null".to_string());
    levels.push("null".to_string());
    if player_types[0] == "cpu" {
        let level1 = prompt_input("What level is the player1 cpu(0/1)?: ");
        levels[2] = String::from(level1);
    }
    if player_types[1] == "cpu" {
        let level2 = prompt_input("What level is the player2 cpu(0/1)?: ");
        levels[3] = String::from(level2);
    }
    return levels.clone();
}
fn get_player_types() -> Vec<String> {
    println!("Welcome to Tic-Tac-Toe!");
    let choice1 = prompt_input("Is player one a cpu or a normal player(cpu/play)?: ");
    let choice2 = prompt_input("Is player two a cpu or a normal player(cpu/play)?: ");
    println!("{}{}", choice1, choice2);
    if (choice1 == "play" || choice1 == "cpu") && (choice2 == "play" || choice2 == "cpu") {
        let output = vec![choice1, choice2];
        return output;
    } else {
        println!("Ivalid");
        return get_player_types();
    }
}
fn is_game_over(board: Vec<Vec<String>>) -> i32 {
    if (board[0][0] == "X" && board[0][1] == "X" && board[0][2] == "X")
        || (board[0][0] == "X" && board[1][0] == "X" && board[2][0] == "X")
        || (board[1][0] == "X" && board[1][1] == "X" && board[1][2] == "X")
        || (board[2][0] == "X" && board[2][1] == "X" && board[2][2] == "X")
        || (board[0][1] == "X" && board[1][1] == "X" && board[2][1] == "X")
        || (board[0][2] == "X" && board[1][2] == "X" && board[2][2] == "X")
        || (board[0][0] == "X" && board[1][1] == "X" && board[2][2] == "X")
        || (board[0][2] == "X" && board[1][1] == "X" && board[2][0] == "X")
    {
        return 0;
    } else if (board[0][0] == "0" && board[0][1] == "0" && board[0][2] == "0")
        || (board[0][0] == "0" && board[1][0] == "0" && board[2][0] == "0")
        || (board[1][0] == "0" && board[1][1] == "0" && board[1][2] == "0")
        || (board[2][0] == "0" && board[2][1] == "0" && board[2][2] == "0")
        || (board[0][1] == "0" && board[1][1] == "0" && board[2][1] == "0")
        || (board[0][2] == "0" && board[1][2] == "0" && board[2][2] == "0")
        || (board[0][0] == "0" && board[1][1] == "0" && board[2][2] == "0")
        || (board[0][2] == "0" && board[1][1] == "0" && board[2][0] == "0")
    {
        return 1;
    } else {
        return 2;
    }
}
fn random_cpu(board: Vec<Vec<String>>, player_icon: String) -> Vec<Vec<String>> {
    let mut counter3 = 0;
    let mut counter4 = 0;
    for array in &board {
        for item in array {
            let random_number = rand::thread_rng().gen_range(0, 6);
            if random_number % 2 == 0 && item == "null" {
                let mut board1 = board.clone();
                board1[counter3][counter4] = player_icon.clone();
                std::thread::sleep(std::time::Duration::from_secs(5));
                return board1;
            }
            counter3 += 1;
        }
        counter4 += 1;
    }
    let mut counter1 = 0;
    let mut counter2 = 0;
    for array in &board {
        for item in array {
            if item == "null" {
                let mut board1 = board.clone();
                board1[counter1][counter2] = player_icon.clone();
                std::thread::sleep(std::time::Duration::from_secs(5));
                return board1;
            }
            counter1 += 1;
        }
        counter2 += 1;
    }
    return board;
}
fn lazy_cpu(board: Vec<Vec<String>>, player_icon: String) -> Vec<Vec<String>> {
    let mut counter1 = 0;
    let mut counter2 = 0;
    for array in &board {
        for item in array {
            if item == "null" {
                let mut board1 = board.clone();
                board1[counter1][counter2] = player_icon.clone();
                std::thread::sleep(std::time::Duration::from_secs(5));
                return board1;
            }
            counter1 += 1;
        }
        counter2 += 1;
    }
    return board;
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
fn make_move(mut board: Vec<Vec<String>>, player_icon: String) -> Vec<Vec<String>> {
    let moves = get_moves();
    let x = moves.0;
    let y = moves.1;
    if &board[x as usize][y as usize] == "null" {
        let mut board1 = board.clone();
        board1[x as usize][y as usize] = player_icon.clone();
        return board1;
    } else {
        println!("{}", "Spot taken!".red());
        board = make_move(board, player_icon);
    }
    return board;
}
