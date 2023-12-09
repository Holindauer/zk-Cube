mod cube;
mod moves;
mod parse;
mod rotation;

fn main() {
    let (scramble, solution) = match parse::parse_args() {
        Ok((first_arg_words, second_arg_words)) => {
            println!("First argument words: {:?}", first_arg_words);
            println!("Second argument words: {:?}", second_arg_words);
            (first_arg_words, second_arg_words)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // Handle the error appropriately, maybe exit the program
            return;
        }
    };

    // Create a solved rubiks cube array
    let mut rubiks_cube = cube::create_solved_cube();

    // Display the cube
    println!("Cube Before Scramble:");
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
}
