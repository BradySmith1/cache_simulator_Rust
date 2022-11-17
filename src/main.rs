mod cache;

use std::fs;
use std::env;
use std::process::exit;
use crate::cache::Cache;

fn cache() {
    let input_string: Vec<String> = read_input_file();
    let mut config_nums: Vec<i32> = vec![];
    for i in 0..3{
        config_nums.push(check_numbers(&input_string[i]));
    }
    check_config(&config_nums);
    let new_cache: Cache = Cache::new(config_nums[0], config_nums[1], config_nums[2]);

}

fn read_input_file() -> Vec<String> {
    let arguments: Vec<String> = env::args().collect();
    let path: String = arguments.get(2).unwrap().to_string();
    let file_contents = fs::read_to_string(&path).expect("Input file invalid.");
    let mut input_strings: Vec<String> = file_contents.lines().map(String::from).collect();
    for i in 0..3{
        input_strings[i] = input_strings[i].chars().last().unwrap().to_string();
    }
    return input_strings;
}

fn check_numbers(set_num: &String) -> i32{
    let mut returnable: i32 = 0;
    match set_num.parse::<i32>() {
        Ok(num) => {
            returnable = num;
        }
        Err(_err) => {
            println!("Cache config numbers are invalid");
            exit(1);
        }
    }
    returnable
}

fn check_config(numbers: &Vec<i32>){
    if numbers[0] > 8000{
        println!("Number of sets exceeds 8000");
        exit(1);
    }
    if numbers[1] > 8{
        println!("Associativity level exceeds 8");
        exit(1);
    }
    if numbers[2] < 4{
        println!("Line size is less than 4");
        exit(1);
    }
    if (numbers[0] % 2) != 0 || (numbers[2] % 2) != 0 {
        println!("Number of sets/line size is not a power of 2");
        exit(1);
    }
}

pub fn main() {
    cache();
}
