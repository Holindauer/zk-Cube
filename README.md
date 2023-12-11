# zk-Cube

Zero-Knowledge Provable Prover of Successful 3x3 Rubiks Cube Solution

# Repository Contents by Directory:

## solution_verifier 
This rust program verifies whether a 3x3 rubik's cube solution is valid or not. It accepts 2 strings as input, one for the cube scramble and the other for the solution. This is done using standard [rubiks cube notation](https://jperm.net/3x3/moves)

Under the hood, the program represents the rubiks cube as a 3D tensor. The solution verifier contains 12 rotation functions that map to the 12 possible moves of a rubiks cube. 

To verify the solution, the scramble is parsed and the corresponding rotation functions are applied to the cube. The solution is verified by applying these rotation functions to the solved cube and checking if the resulting cube is in the solved pposition.

To Run the program from the command line, within the solution_verifier directory, run the following command:

    cargo build --release
    ./target/release/solution_verifier <scramble> <solution>

scramble and solution are strings of the form "R U R' U' R' F R2 U' R' U' R U R' F'". They must be encapsulated in quotes because they are split on whitespace. The program will print "Valid Solution" if the solution is valid and "Invalid Solution" if the solution is invalid.

## zk_solution_verifier

This directory contains an implementation of the solution verifier as a zk-STARK using [risc0](https://github.com/risc0/risc0). This means that the execution of the solution_verifier itself is verified with a [zero knowledge proof](https://en.wikipedia.org/wiki/Non-interactive_zero-knowledge_proof) without revealing your solution, the scramble it solves, or any state during execution. In the case of risc0, the generated proof is called the receipt. It is cryptographically infeasible to generate a valid receipt using risc0 unless the execution of the program is valid.

## Risc0 File Structure
Risc0 emulates a RISC-V processor. In which, the solution_verifier rust program is compiled into a RISC-V ELF file from the [zk_solution_verifier/methods/guest](zk_solution_verifier/methods/guest/src). The host program runs and proves the guest solution_verifier executable inside the zkVM. This is where the receipt of the proof is generated. This receipt provides proof of execution.

 This file is executed within the guest, an emulated RISC-V computer. 
.  
├── Cargo.lock  
├── Cargo.toml  
├── LICENSE  
├── README.md  
├── host  
│   ├── Cargo.toml  
│   └── src  
│       └── main.rs  <------------- Here is the host program  
├── methods  
│   ├── Cargo.toml  
│   ├── build.rs  
│   ├── guest  
│   │   ├── Cargo.lock  
│   │   ├── Cargo.toml  
│   │   ├── src    <------------- Here is the source code for the solution verifier  
│   │   │   ├── cube.rs  
│   │   │   ├── main.rs  
│   │   │   ├── moves.rs  
│   │   │   ├── parse.rs  
│   │   │   ├── rotation.rs  
│   │   │   └── verify.rs  
│   │   └── target  


## Running the zk_solution_verifier

To run the zk_solution_verifier, you must first install rust, cargo, and risc0.

    curl https://sh.rustup.rs -sSf | sh
    cargo install cargo-binstall
    cargo binstall cargo-risczero

Currently, in order to input your solution and scramble, modify the string vector in zk_solution_verifier/host/src/main.rs. The first string is the scramble and the second string is the solution. 

    let args = vec!["R U R' U' R' F R2 U' R' U' R U R' F'".to_string(), "R U R' U' R' F R2 U' R' U' R U R' F'".to_string()];

By calling this command within the zk_solution_verifier directory, the program will begin complition, the guest will be run within the host, and the receipt will be generated.
    
        cargo run --release


    