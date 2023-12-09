// parse.rs is used to parse two command line arguments (scrambled and solution) for the program into a list of strings that will be used to 
// first scramble the rubiks cube and then verify if the solution is correct.

use std::env;
use std::error::Error;
use crate::moves;


pub fn parse_args() -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip the program name

    // Check if there are exactly two arguments
    if args.len() != 2 {
        return Err("Exactly two arguments are required.".into());
    }

    // Split each argument into words
    let first_arg_words: Vec<String> = args[0].split_whitespace().map(String::from).collect();
    let second_arg_words: Vec<String> = args[1].split_whitespace().map(String::from).collect();

    Ok((first_arg_words, second_arg_words))
}


// make_move() contains a switch statement that recieves the array of strings arguments and then calls the appropriate move function
pub fn make_move(cube: &mut [[[i32; 5]; 5]; 5], move_string: &str) {
    match move_string {
        "U" => moves::U(cube, 'c'),
        "U'" => moves::U(cube, 'p'),
        "B" => moves::B(cube, 'c'),
        "B'" => moves::B(cube, 'p'),
        "D" => moves::D(cube, 'c'),
        "D'" => moves::D(cube, 'p'),
        "L" => moves::L(cube, 'c'),
        "L'" => moves::L(cube, 'p'),
        "F" => moves::F(cube, 'c'),
        "F'" => moves::F(cube, 'p'),
        "R" => moves::R(cube, 'c'),
        "R'" => moves::R(cube, 'p'),
        _ => println!("Invalid Move"),
    }
}