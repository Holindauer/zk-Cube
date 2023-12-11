#![no_main]

// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental

mod cube;
mod moves;
mod parse;
mod rotation;
mod verify;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {

    // read the input
    let input: Vec<String> = env::read();

    // Parse the command line arguments ----> 2 string vectors containing the scramble and solution
    let parsed_result = parse::parse_args(input.clone());

    // Handle the Result manually
    let (scramble, solution) = match parsed_result {
        Ok((scramble, solution)) => (scramble, solution),
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            return; // Exit the function early
        }
    };

    // Create a solved 3x3 rubiks cube array --- 5x5x5 in memory
    let mut rubiks_cube = cube::create_solved_cube();

    // Display the cube before scrambling
    println!("\nCube Before Scramble:");
    cube::display_cube(&rubiks_cube);

    // iterate through the scramble vector, passing each string to the make_move function
    for move_str in &scramble {
        parse::make_move(&mut rubiks_cube, move_str);
    }

    // Display the scrambled cube
    println!("Scrambled Cube:");
    cube::display_cube(&rubiks_cube);

    // iterate through the solution vector, passing each string to the make_move function
    for move_str in &solution {
        parse::make_move(&mut rubiks_cube, move_str);
    }

    // Display the cube after applying the solution
    println!("Cube After Applying Solution:");
    cube::display_cube(&rubiks_cube);

    // Check if the cube is solved
    if verify::is_solved(&rubiks_cube) {
        println!("Cube is solved!");
    } else {
        println!("Cube is not solved!");
    }


    // write public output to the journal
    env::commit(&input);
}
