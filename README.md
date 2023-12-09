# zk-Cube

Zero Knowledge Proof of Successful Cube Solution

# Current Repository Contents:

## solution_verifier 
This rust program verifies whether a 3x3 rubik's cube solution is valid or not. It accepts 2 strings as input, one for the cube scramble and the other for the solution. This is done using standard [rubiks cube notation](https://jperm.net/3x3/moves)

Under the hood, the program represents the rubiks cube as a 3D tensor. The solution verifier contains 12 rotation functions that map to the 12 possible moves of a rubiks cube. 

To verify the solution, the scramble is parsed and the corresponding rotation functions are applied to the cube. The solution is verified by applying these rotation functions to the solved cube and checking if the resulting cube is in the solved pposition.

To Run the program from the command line, within the solution_verifier directory, run the following command:

    cargo build --release
    ./target/release/solution_verifier <scramble> <solution>

<scramble> and <solution> are strings of the form "R U R' U' R' F R2 U' R' U' R U R' F'". The program will print "Valid Solution" if the solution is valid and "Invalid Solution" if the solution is invalid.