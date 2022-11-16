use std::fs::read;
use std::io;
use std::io::Write;

fn cache(){
    let input = io::stdin();
    let mut set_num: String = String::new();
    let mut set_size = String::new();
    let mut line_size = String::new();
    set_num = read_input("Number of sets: ".to_string());
    set_size = read_input("Set size: ".to_string());
    line_size = read_input("Line size: ".to_string());
    for i in 0..8 {

    }
}

fn read_input(prompt: String) -> String{
    done = false;
    let mut input_string = String::new();
    let input = io::stdin();
    while done == false {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        input.read_line(&mut input_string).expect("Failed to read line");
        match line_size.trim().parse::<i32>() {
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
