mod moves;
mod rotation;
mod cube;


fn main() {


    // Create a solved rubiks cube array
    let mut rubiks_cube = cube::create_solved_cube();

    // Display the cube
    println!("Solved Cube:");
    cube::display_cube(&rubiks_cube);

    // Perform the following moves: U U R F L R' U' D B D' B
    moves::U(&mut rubiks_cube, 'c');
    moves::U(&mut rubiks_cube, 'c');
    moves::R(&mut rubiks_cube, 'c');
    moves::F(&mut rubiks_cube, 'c');

    moves::L(&mut rubiks_cube, 'c');
    moves::R(&mut rubiks_cube, 'p');
    moves::U(&mut rubiks_cube, 'p');

    moves::D(&mut rubiks_cube, 'c');
    moves::B(&mut rubiks_cube, 'c');
    moves::D(&mut rubiks_cube, 'p');
    moves::B(&mut rubiks_cube, 'p');

    // Display the cube
    println!("Scrambled Cube:");
    cube::display_cube(&rubiks_cube);

    // Reverse the scramble
    moves::B(&mut rubiks_cube, 'c');
    moves::D(&mut rubiks_cube, 'c');
    moves::B(&mut rubiks_cube, 'p');
    moves::D(&mut rubiks_cube, 'p');

    moves::U(&mut rubiks_cube, 'c');
    moves::R(&mut rubiks_cube, 'c');
    moves::L(&mut rubiks_cube, 'p');

    moves::F(&mut rubiks_cube, 'p');
    moves::R(&mut rubiks_cube, 'p');
    moves::U(&mut rubiks_cube, 'p');
    moves::U(&mut rubiks_cube, 'p');

    // Display the cube
    println!("Solved Cube:");
    cube::display_cube(&rubiks_cube);


}
