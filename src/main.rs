extern crate core;

mod cache;

use crate::cache::Cache;
use std::env;
use std::fs;
use std::process::exit;
//! File that runs the simulation of a cache.
//!
//! Author: Michael Imerman and Brady Smith
//! Version: 1.0.0
//!

fn run() {
    /// Function that calls various local methods and creates a new cache to access.
    /// Prints results of access at the end.
    ///
    let mut instruction_sets: Vec<String> = read_input_file();
    let mut config_nums: Vec<i32> = vec![];
    for _ in 0..3 {
        config_nums.push(check_numbers(&instruction_sets[0]));
        instruction_sets.remove(0);
    }
    check_config(&config_nums);
    let mut cache: Cache = Cache::new(config_nums[0], config_nums[1], config_nums[2], 0.0, 0.0);
    for i in instruction_sets {
        let instructions: Vec<String> = split_instruction(i);
        cache.access(&instructions[0], &instructions[1], &instructions[2]);
    }
    println!("{}", cache.to_string());
    println!("{}", cache.summary());
}

fn split_instruction(instruction: String) -> Vec<String> {
    /// Splits the instruction that is input to grab the values between a : and returns the
    /// instruction.
    ///
    /// # Arguments
    ///
    /// instruction - An array of strings that is returned to hold the whole instruction.
    ///
    let new_instruction: Vec<&str> = instruction.split_terminator(":").collect();
    let new_instruction: Vec<String> = new_instruction.iter().map(|x| x.to_string()).collect();
    return new_instruction;
}

fn read_input_file() -> Vec<String> {
    /// Allows the reading of the input file and describes how to run if the original input
    /// was incorrect.
    /// Grabs the values and returns an array of strings.
    /// Does some error handling for if the file input is invalid.
    ///
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 3 {
        println!("Usage: cargo run -- -f <input_file>");
        exit(1);
    }
    let path: String = arguments.get(2).unwrap().to_string();
    let file_contents = fs::read_to_string(&path);
    let file_contents = match file_contents {
        Ok(file_contents) => file_contents,
        Err(_) => {
            println!("Error: File not found");
            exit(1);
        }
    };
    let mut input_strings: Vec<String> = file_contents.lines().map(String::from).collect();
    for i in 0..3 {
        input_strings[i] = input_strings[i].split(" ").last().unwrap().to_string();
    }
    return input_strings;
}


fn check_numbers(set_num: &String) -> i32 {
    /// Checks to ensure that the number of sets is valid and prints a message if is not.
    let returnable: i32;
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

fn check_config(numbers: &Vec<i32>) {
    /// Does various checks for the configuration of the input file to ensure the cache
    /// can be modeled correctly.
    ///
    /// # Arguments
    ///
    /// numbers - An array of numbers that represents various parts of the input file like \
    /// number of sets, associativity level, line size of the input file, and if sets/line size
    /// is a power of 2.
    ///
    if numbers[0] > 8000 {
        println!("Number of sets exceeds 8000");
        exit(1);
    }
    if numbers[1] > 8 {
        println!("Associativity level exceeds 8");
        exit(1);
    }
    if numbers[2] < 4 {
        println!("Line size is less than 4");
        exit(1);
    }
    if (numbers[0] % 2) != 0 || (numbers[2] % 2) != 0 {
        println!("Number of sets/line size is not a power of 2");
        exit(1);
    }
}

pub fn main() {
/// Main function that calls the run function.
    run();
}
