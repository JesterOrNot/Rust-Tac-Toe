use colored::*;
use rand::Rng;
use std::io::Write;

fn main() {
    let mut board = new_board();
    let data = main_menu();
    let mut player_one_turn = true;
    print_board(board.clone());
    loop {
        if player_one_turn {
            if data[0] == "cpu" && data[2] == "0" {
                board = lazy_cpu(board.clone(), String::from("X"));
            } else if data[0] == "cpu" && data[2] == "1" {
                board = random_cpu(board.clone(), String::from("X"));
            } else {
                board = make_move(board.clone(), String::from("X"));
            }
            print_board(board.clone());
            player_one_turn = false;
        } else {
            if data[1] == "cpu" && data[3] == "0" {
                board = lazy_cpu(board.clone(), String::from("0"));
            } else if data[1] == "cpu" && data[3] == "1" {
                board = random_cpu(board.clone(), String::from("0"));
            } else {
                make_move(board.clone(), String::from("0"));
            }
            print_board(board.clone());
            player_one_turn = true;
        }
        let is_true = is_draw(board.clone());
        let is_over = is_game_over(board.clone());
        if is_true {
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("\nIt's A Tie!");
            break;
        }
        if is_over == 1 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("\nPlayer 2 Wins!");
            break;
        } else if is_over == 0 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("\nPlayer 1 Wins!");
            break;
        }
    }
    again_or_no();
}
fn again_or_no() {
    let choice = prompt_input("Do you want to play again(y/n)?: ");
    if choice == "y" {
        main();
    } else {
        println!("Ok, Bye!");
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
                    print!("{}", "0".red())
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
    return (item1 - 1, item2 - 1);
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
    for counter1 in 0..3 {
        for counter2 in 0..3 {
            let random_number = rand::thread_rng().gen_range(0, 6);
            if random_number % 2 == 0 && board[counter1][counter2] == "null" {
                let mut board1 = board.clone();
                board1[counter1][counter2] = player_icon.clone();
                println!("Thinking.....");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return board1;
            }
        }
    }
    lazy_cpu(board.clone(), player_icon.clone());
    return board;
}
fn lazy_cpu(board: Vec<Vec<String>>, player_icon: String) -> Vec<Vec<String>> {
    for counter1 in 0..3 {
        for counter2 in 0..3 {
            if board[counter1][counter2] == "null" {
                let mut board1 = board.clone();
                board1[counter1][counter2] = player_icon.clone();
                println!("Thinking.....");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return board1;
            }
        }
    }
    return board;
}
fn is_draw(board: Vec<Vec<String>>) -> bool {
    let mut count = 0;
    for array in board {
        for item in array {
            if item == "X" || item == "0" {
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
    let x = moves.1;
    let y = moves.0;
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
