use std::fs::read;
use std::io;
use std::io::Write;

fn cache(){
    let mut set_num: String = read_input("Number of sets: ".to_string());
    let mut set_size: String = read_input("Set size: ".to_string());
    let mut line_size: String = read_input("Line size: ".to_string());
    for i in 0..8 {

    }
}

fn read_input(prompt: String) -> String{
    let mut done = false;
    let mut input_string = String::new();
    let input = io::stdin();
    while done == false {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        input.read_line(&mut input_string).expect("Failed to read line");
        match input_string.trim().parse::<i32>() {
            Ok(_) => {
                done = true;
            }
            Err(_err) => {
                println!("Invalid input");
                input_string.clear();
            }
        };
    }
    return input_string;
}

fn main() {
    cache();
}
