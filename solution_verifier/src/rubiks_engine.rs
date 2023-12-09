
pub fn B(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    
}







// This function performs the B move on the rubiks cube array
pub fn Perform_B(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the B face
    let B_FACE_upper_left_corner = cube[4][1][3];
    let B_FACE_upper_right_corner = cube[4][1][1];
    let B_FACE_lower_left_corner = cube[4][3][3];
    let B_FACE_lower_right_corner = cube[4][3][1];

    // Get Edges of the B face
    let B_FACE_upper_edge = cube[4][1][2];
    let B_FACE_left_edge = cube[4][2][3];
    let B_FACE_lower_edge = cube[4][3][2];
    let B_FACE_right_edge = cube[4][2][1];

    // Get the Connected Edges of the B face
    let B_upper_SIDE_left_edge = cube[3][0][3];
    let B_upper_SIDE_middle_edge = cube[3][0][2];
    let B_upper_SIDE_right_edge = cube[3][0][1];

    let B_left_SIDE_upper_edge = cube[3][1][4];
    let B_left_SIDE_middle_edge = cube[3][2][4];
    let B_left_SIDE_lower_edge = cube[3][3][4]; 

    let B_lower_SIDE_left_edge = cube[3][4][3];
    let B_lower_SIDE_middle_edge = cube[3][4][2];
    let B_lower_SIDE_right_edge = cube[3][4][1];

    let B_right_SIDE_upper_edge = cube[3][1][0];
    let B_right_SIDE_middle_edge = cube[3][2][0];
    let B_right_SIDE_lower_edge = cube[3][3][0];
    
    // Perform the B move for the B face 

    // Rotate Corners Clockwise on the B face
    cube[4][1][3] = B_FACE_lower_left_corner; // B_FACE_upper_left_corner <---- B_FACE_lower_left_corner
    cube[4][1][1] = B_FACE_upper_left_corner; // B_FACE_upper_right_corner <---- B_FACE_upper_left_corner
    cube[4][3][3] = B_FACE_lower_right_corner; // B_FACE_lower_left_corner <---- B_FACE_lower_right_corner
    cube[4][3][1] = B_FACE_upper_right_corner; // B_FACE_lower_right_corner <---- B_FACE_upper_right_corner

    // Rotate Edges Clockwise on the B face
    cube[4][1][2] = B_FACE_left_edge; // B_FACE_upper_edge <---- B_FACE_left_edge
    cube[4][2][3] = B_FACE_lower_edge; // B_FACE_left_edge <---- B_FACE_lower_edge
    cube[4][3][2] = B_FACE_right_edge; // B_FACE_lower_edge <---- B_FACE_right_edge
    cube[4][2][1] = B_FACE_upper_edge; // B_FACE_right_edge <---- B_FACE_upper_edge

    // // // Rotate Connected Edges Clockwise on the D face
    cube[3][0][3] = B_left_SIDE_lower_edge; // B_upper_SIDE_left_edge <---- B_left_SIDE_lower_edge
    cube[3][0][2] = B_left_SIDE_middle_edge; // B_upper_SIDE_middle_edge <---- B_left_SIDE_middle_edge
    cube[3][0][1] = B_left_SIDE_upper_edge; // B_upper_SIDE_right_edge <---- B_left_SIDE_upper_edge

    cube[3][1][4] = B_lower_SIDE_left_edge; // B_left_SIDE_upper_edge <---- B_lower_SIDE_left_edge
    cube[3][2][4] = B_lower_SIDE_middle_edge; // B_left_SIDE_middle_edge <---- B_lower_SIDE_middle_edge
    cube[3][3][4] = B_lower_SIDE_right_edge; // B_left_SIDE_lower_edge <---- B_lower_SIDE_right_edge

    cube[3][4][3] = B_right_SIDE_lower_edge; // B_lower_SIDE_left_edge <---- B_right_SIDE_lower_edge
    cube[3][4][2] = B_right_SIDE_middle_edge; // B_lower_SIDE_middle_edge <---- B_right_SIDE_middle_edge
    cube[3][4][1] = B_right_SIDE_upper_edge; //B_lower_SIDE_right_edge <---- B_right_SIDE_upper_edge

    cube[3][1][0] = B_upper_SIDE_left_edge; // B_right_SIDE_upper_edge <--- B_upper_SIDE_left_edge
    cube[3][2][0] = B_upper_SIDE_middle_edge; // B_right_SIDE_middle_edge <--- B_upper_SIDE_middle_edge
    cube[3][3][0] = B_upper_SIDE_right_edge; // B_right_SIDE_lower_edge  <--- B_upper_SIDE_right_edge
}

// This function performs the B move on the rubiks cube array
pub fn Perform_B_Prime(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the B face
    let B_FACE_upper_left_corner = cube[4][1][3];
    let B_FACE_upper_right_corner = cube[4][1][1];
    let B_FACE_lower_left_corner = cube[4][3][3];
    let B_FACE_lower_right_corner = cube[4][3][1];

    // Get Edges of the B face
    let B_FACE_upper_edge = cube[4][1][2];
    let B_FACE_left_edge = cube[4][2][3];
    let B_FACE_lower_edge = cube[4][3][2];
    let B_FACE_right_edge = cube[4][2][1];

    // Get the Connected Edges of the B face
    let B_upper_SIDE_left_edge = cube[3][0][3];
    let B_upper_SIDE_middle_edge = cube[3][0][2];
    let B_upper_SIDE_right_edge = cube[3][0][1];

    let B_left_SIDE_upper_edge = cube[3][1][4];
    let B_left_SIDE_middle_edge = cube[3][2][4];
    let B_left_SIDE_lower_edge = cube[3][3][4]; 

    let B_lower_SIDE_left_edge = cube[3][4][3];
    let B_lower_SIDE_middle_edge = cube[3][4][2];
    let B_lower_SIDE_right_edge = cube[3][4][1];

    let B_right_SIDE_upper_edge = cube[3][1][0];
    let B_right_SIDE_middle_edge = cube[3][2][0];
    let B_right_SIDE_lower_edge = cube[3][3][0];
    
    // Perform the B" move for the B face 

    // Rotate Corners Counter Clockwise on the B face
    cube[4][1][3] = B_FACE_upper_right_corner; // B_FACE_upper_left_corner <---- B_FACE_upper_right_corner
    cube[4][1][1] = B_FACE_lower_right_corner; // B_FACE_upper_right_corner <---- B_FACE_lower_right_corner
    cube[4][3][1] = B_FACE_lower_left_corner; // B_FACE_lower_right_corner <---- B_FACE_lower_left_corner
    cube[4][3][3] = B_FACE_upper_left_corner; // B_FACE_lower_left_corner <---- B_FACE_upper_left_corner
 

    // Rotate Edges Counter Clockwise on the B face
    cube[4][1][2] = B_FACE_right_edge; // B_FACE_upper_edge <---- B_FACE_right_edge
    cube[4][2][3] = B_FACE_upper_edge; // B_FACE_left_edge <---- B_FACE_upper_edge
    cube[4][3][2] = B_FACE_left_edge; // B_FACE_lower_edge <---- B_FACE_left_edge
    cube[4][2][1] = B_FACE_lower_edge; // B_FACE_right_edge <---- B_FACE_lower_edge

    // Rotate Connected Edges Counter Clockwise on the B face
    cube[3][0][3] = B_right_SIDE_upper_edge; // B_upper_SIDE_left_edge <---- B_right_SIDE_upper_edge 
    cube[3][0][2] = B_right_SIDE_middle_edge; // B_upper_SIDE_middle_edge <---- B_right_SIDE_middle_edge
    cube[3][0][1] = B_right_SIDE_lower_edge; // B_upper_SIDE_right_edge <---- B_right_SIDE_lower_edge

    cube[3][1][4] = B_upper_SIDE_right_edge; // B_left_SIDE_upper_edge <----  B_upper_SIDE_right_edge
    cube[3][2][4] = B_upper_SIDE_middle_edge; // B_left_SIDE_middle_edge <---- B_upper_SIDE_middle_edge
    cube[3][3][4] = B_upper_SIDE_left_edge; // B_left_SIDE_lower_edge <---- B_upper_SIDE_left_edge

    cube[3][4][3] = B_left_SIDE_upper_edge; // B_lower_SIDE_left_edge <---- B_left_SIDE_upper_edge
    cube[3][4][2] = B_left_SIDE_middle_edge; // B_lower_SIDE_middle_edge <---- B_left_SIDE_middle_edge
    cube[3][4][1] = B_left_SIDE_lower_edge; //B_lower_SIDE_right_edge <---- B_left_SIDE_lower_edge 

    cube[3][1][0] = B_lower_SIDE_right_edge; // B_right_SIDE_upper_edge <--- B_lower_SIDE_right_edge
    cube[3][2][0] = B_lower_SIDE_middle_edge; // B_right_SIDE_middle_edge <--- B_lower_SIDE_middle_edge
    cube[3][3][0] = B_lower_SIDE_left_edge; // B_right_SIDE_lower_edge  <---  B_lower_SIDE_left_edge 
}

// This function performs the D move on the rubiks cube array
pub fn Perform_D(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the D face
    let D_FACE_upper_left_corner = cube[1][4][1];
    let D_FACE_upper_right_corner = cube[1][4][3];
    let D_FACE_lower_left_corner = cube[3][4][1];
    let D_FACE_lower_right_corner = cube[3][4][3];

    // Get Edges of the D face
    let D_upper_edge = cube[1][4][2];
    let D_left_edge = cube[2][4][1];
    let D_lower_edge = cube[3][4][2];
    let D_right_edge = cube[2][4][3]; 

    // Get the Connected Edges of the D face
    let D_upper_SIDE_left_edge = cube[0][3][1];
    let D_upper_SIDE_middle_edge = cube[0][3][2];
    let D_upper_SIDE_right_edge = cube[0][3][3];

    let D_left_SIDE_upper_edge = cube[1][3][0];
    let D_left_SIDE_middle_edge = cube[2][3][0];
    let D_left_SIDE_lower_edge = cube[3][3][0];

    let D_lower_SIDE_left_edge = cube[4][3][1];
    let D_lower_SIDE_middle_edge = cube[4][3][2];
    let D_lower_SIDE_right_edge = cube[4][3][3];

    let D_right_SIDE_upper_edge = cube[1][3][4];
    let D_right_SIDE_middle_edge = cube[2][3][4];
    let D_right_SIDE_lower_edge = cube[3][3][4];
    
    // Perform the D move for the D face 

    // Rotate Corners Clockwise on the L face
    cube[1][4][1] = D_FACE_lower_left_corner; // D_FACE_upper_left_corner <---- D_FACE_lower_left_corner
    cube[1][4][3] = D_FACE_upper_left_corner; // D_FACE_upper_right_corner <---- D_FACE_upper_left_corner
    cube[3][4][3] = D_FACE_upper_right_corner; // D_FACE_lower_right_corner <---- D_FACE_upper_right_corner
    cube[3][4][1] = D_FACE_lower_right_corner; // D_FACE_lower_left_corner <---- D_FACE_lower_right_corner

    // Rotate Edges Clockwise on the D face
    cube[1][4][2] = D_left_edge; // D_upper_edge <---- D_left_edge
    cube[2][4][1] = D_lower_edge; // D_left_edge <---- D_lower_edge
    cube[3][4][2] = D_right_edge; // D_lower_edge <---- D_right_edge
    cube[2][4][3] = D_upper_edge; // D_right_edge <---- D_upper_edge

    // // Rotate Connected Edges Clockwise on the D face
    cube[0][3][1] = D_left_SIDE_lower_edge; // D_upper_SIDE_left_edge <---- D_left_SIDE_lower_edge
    cube[0][3][2] = D_left_SIDE_middle_edge; // D_upper_SIDE_middle_edge <---- D_left_SIDE_middle_edge
    cube[0][3][3] = D_left_SIDE_upper_edge; // D_upper_SIDE_right_edge <---- D_left_SIDE_upper_edge

    cube[1][3][0] = D_lower_SIDE_left_edge; // D_left_SIDE_upper_edge <---- D_lower_SIDE_left_edge
    cube[2][3][0] = D_lower_SIDE_middle_edge; // D_left_SIDE_middle_edge <---- D_lower_SIDE_middle_edge
    cube[3][3][0] = D_lower_SIDE_right_edge; // D_left_SIDE_lower_edge <---- D_lower_SIDE_right_edge

    cube[4][3][1] = D_right_SIDE_lower_edge; // D_lower_SIDE_left_edge <---- D_right_SIDE_lower_edge
    cube[4][3][2] = D_right_SIDE_middle_edge; // D_lower_SIDE_middle_edge <---- D_right_SIDE_middle_edge
    cube[4][3][3] = D_right_SIDE_upper_edge; // D_lower_SIDE_right_edge <---- D_right_SIDE_upper_edge

    cube[1][3][4] = D_upper_SIDE_left_edge; // D_right_SIDE_upper_edge <--- D_upper_SIDE_left_edge
    cube[2][3][4] = D_upper_SIDE_middle_edge; // D_right_SIDE_middle_edge <--- D_upper_SIDE_middle_edge 
    cube[3][3][4] = D_upper_SIDE_right_edge; // D_right_SIDE_lower_edge <--- D_upper_SIDE_right_edge
}

// This function performs the D' move on the rubiks cube array
pub fn Perform_D_Prime(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the D face
    let D_FACE_upper_left_corner = cube[1][4][1];
    let D_FACE_upper_right_corner = cube[1][4][3];
    let D_FACE_lower_left_corner = cube[3][4][1];
    let D_FACE_lower_right_corner = cube[3][4][3];

    // Get Edges of the D face
    let D_upper_edge = cube[1][4][2];
    let D_left_edge = cube[2][4][1];
    let D_lower_edge = cube[3][4][2];
    let D_right_edge = cube[2][4][3]; 

    // Get the Connected Edges of the D face
    let D_upper_SIDE_left_edge = cube[0][3][1];
    let D_upper_SIDE_middle_edge = cube[0][3][2];
    let D_upper_SIDE_right_edge = cube[0][3][3];

    let D_left_SIDE_upper_edge = cube[1][3][0];
    let D_left_SIDE_middle_edge = cube[2][3][0];
    let D_left_SIDE_lower_edge = cube[3][3][0];

    let D_lower_SIDE_left_edge = cube[4][3][1];
    let D_lower_SIDE_middle_edge = cube[4][3][2];
    let D_lower_SIDE_right_edge = cube[4][3][3];

    let D_right_SIDE_upper_edge = cube[1][3][4];
    let D_right_SIDE_middle_edge = cube[2][3][4];
    let D_right_SIDE_lower_edge = cube[3][3][4];
    
    // Perform the D' move for the D face 

    // Rotate Corners Counter Clockwise on the L face
    cube[1][4][1] = D_FACE_upper_right_corner; // D_FACE_upper_left_corner <---- D_FACE_upper_right_corner
    cube[1][4][3] = D_FACE_lower_right_corner; // D_FACE_upper_right_corner <---- D_FACE_lower_right_corner
    cube[3][4][3] = D_FACE_lower_left_corner; // D_FACE_lower_right_corner <---- D_FACE_lower_left_corner
    cube[3][4][1] = D_FACE_upper_left_corner; // D_FACE_lower_left_corner <---- D_FACE_upper_left_corner

    // Rotate Edges Counter Clockwise on the D face
    cube[1][4][2] = D_right_edge; // D_upper_edge <---- D_right_edge
    cube[2][4][1] = D_upper_edge; // D_left_edge <---- D_upper_edge
    cube[3][4][2] = D_left_edge; // D_lower_edge <---- D_left_edge
    cube[2][4][3] = D_lower_edge; // D_right_edge <---- D_lower_edge

    // Rotate Connected Edges Counter Clockwise on the D face
    cube[0][3][1] = D_right_SIDE_upper_edge; // D_upper_SIDE_left_edge <---- D_right_SIDE_upper_edge
    cube[0][3][2] = D_right_SIDE_middle_edge; // D_upper_SIDE_middle_edge <---- D_right_SIDE_middle_edge
    cube[0][3][3] = D_right_SIDE_lower_edge; // D_upper_SIDE_right_edge <---- D_right_SIDE_lower_edge

    cube[1][3][0] = D_upper_SIDE_right_edge; // D_left_SIDE_upper_edge <---- D_upper_SIDE_right_edge
    cube[2][3][0] = D_upper_SIDE_middle_edge; // D_left_SIDE_middle_edge <---- D_upper_SIDE_middle_edge 
    cube[3][3][0] = D_upper_SIDE_left_edge; // D_left_SIDE_lower_edge <----  D_upper_SIDE_left_edge

    cube[4][3][1] = D_left_SIDE_upper_edge; // D_lower_SIDE_left_edge <---- D_left_SIDE_upper_edge
    cube[4][3][2] = D_left_SIDE_middle_edge; // D_lower_SIDE_middle_edge <---- D_left_SIDE_middle_edge
    cube[4][3][3] = D_left_SIDE_lower_edge; // D_lower_SIDE_right_edge <---- D_left_SIDE_lower_edge

    cube[1][3][4] = D_lower_SIDE_right_edge; // D_right_SIDE_upper_edge <--- D_lower_SIDE_right_edge
    cube[2][3][4] = D_lower_SIDE_middle_edge; // D_right_SIDE_middle_edge <--- D_lower_SIDE_middle_edge  
    cube[3][3][4] = D_lower_SIDE_left_edge; // D_right_SIDE_lower_edge <--- D_lower_SIDE_left_edge 
}

// This function performs the L move on the rubiks cube array
pub fn Perform_L(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the L face
    let L_FACE_upper_left_corner = cube[3][1][0];
    let L_FACE_upper_right_corner = cube[1][1][0];
    let L_FACE_lower_right_corner = cube[1][3][0];
    let L_FACE_lower_left_corner = cube[3][3][0];

    // Get Edges of the L face
    let L_FACE_upper_edge = cube[2][1][0];
    let L_FACE_right_edge = cube[1][2][0];
    let L_FACE_lower_edge = cube[2][3][0];
    let L_FACE_left_edge = cube[3][2][0];

    // // Get the Connected Edges of the F face
    let L_upper_SIDE_left_edge = cube[3][0][1];
    let L_upper_SIDE_middle_edge = cube[2][0][1];
    let L_upper_SIDE_right_edge = cube[1][0][1];

    let L_left_SIDE_upper_edge = cube[4][1][1];
    let L_left_SIDE_middle_edge = cube[4][2][1];
    let L_left_SIDE_lower_edge = cube[4][3][1];

    let L_lower_SIDE_left_edge = cube[3][4][1];
    let L_lower_SIDE_middle_edge = cube[2][4][1];
    let L_lower_SIDE_right_edge = cube[1][4][1];

    let L_right_SIDE_upper_edge = cube[0][1][1];
    let L_right_SIDE_middle_edge = cube[0][2][1];
    let L_right_SIDE_lower_edge = cube[0][3][1];
    
    // Perform the L move for the L face 

    // Rotate Corners Clockwise on the L face
    cube[3][1][0]; // L_FACE_upper_left_corner <---- L_FACE_lower_left_corner
    cube[1][1][0]; // L_FACE_upper_right_corner <---- L_FACE_upper_left_corner
    cube[1][3][0]; // L_FACE_lower_right_corner <---- L_FACE_upper_right_corner
    cube[3][3][0]; // L_FACE_lower_left_corner <---- L_FACE_lower_right_corner

    
    // Rotate Edges Clockwise on the L face
    cube[2][1][0] = L_FACE_left_edge; // L_FACE_upper_edge <---- L_FACE_left_edge
    cube[1][2][0] = L_FACE_upper_edge; // L_FACE_right_edge <---- L_FACE_upper_edge
    cube[2][3][0] = L_FACE_right_edge; // L_FACE_lower_edge <---- L_FACE_right_edge
    cube[3][2][0] = L_FACE_lower_edge; // L_FACE_left_edge <---- L_FACE_lower_edge

    // // Rotate Connected Edges Clockwise on the L face
    cube[3][0][1] = L_left_SIDE_lower_edge; // L_upper_SIDE_left_edge <---- L_left_SIDE_lower_edge
    cube[2][0][1] = L_left_SIDE_middle_edge; // L_upper_SIDE_middle_edge <---- L_left_SIDE_middle_edge
    cube[1][0][1] = L_left_SIDE_upper_edge; // L_upper_SIDE_right_edge <---- L_left_SIDE_upper_edge

    cube[4][1][1] = L_lower_SIDE_left_edge; // L_left_SIDE_upper_edge <---- L_lower_SIDE_left_edge
    cube[4][2][1] = L_lower_SIDE_middle_edge; // L_left_SIDE_middle_edge <---- L_lower_SIDE_middle_edge
    cube[4][3][1] = L_lower_SIDE_right_edge; // L_left_SIDE_lower_edge <---- L_lower_SIDE_right_edge

    cube[3][4][1] = L_right_SIDE_lower_edge; // L_lower_SIDE_left_edge <---- L_right_SIDE_lower_edge
    cube[2][4][1] = L_right_SIDE_middle_edge; // L_lower_SIDE_middle_edge <---- L_right_SIDE_middle_edge
    cube[1][4][1] = L_right_SIDE_upper_edge; // L_lower_SIDE_right_edge <---- L_right_SIDE_upper_edge

    cube[0][1][1] = L_upper_SIDE_left_edge; // L_right_SIDE_upper_edge <--- L_upper_SIDE_left_edge
    cube[0][2][1] = L_upper_SIDE_middle_edge; // L_right_SIDE_middle_edge <--- L_upper_SIDE_middle_edge
    cube[0][3][1] = L_upper_SIDE_right_edge; // L_right_SIDE_lower_edge <--- L_upper_SIDE_right_edge
}

// This function performs the L move on the rubiks cube array
pub fn Perform_L_Prime(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the L face
    let L_FACE_upper_left_corner = cube[3][1][0];
    let L_FACE_upper_right_corner = cube[1][1][0];
    let L_FACE_lower_right_corner = cube[1][3][0];
    let L_FACE_lower_left_corner = cube[3][3][0];

    // Get Edges of the L face
    let L_FACE_upper_edge = cube[2][1][0];
    let L_FACE_right_edge = cube[1][2][0];
    let L_FACE_lower_edge = cube[2][3][0];
    let L_FACE_left_edge = cube[3][2][0];

    // // Get the Connected Edges of the F face
    let L_upper_SIDE_left_edge = cube[3][0][1];
    let L_upper_SIDE_middle_edge = cube[2][0][1];
    let L_upper_SIDE_right_edge = cube[1][0][1];

    let L_left_SIDE_upper_edge = cube[4][1][1];
    let L_left_SIDE_middle_edge = cube[4][2][1];
    let L_left_SIDE_lower_edge = cube[4][3][1];

    let L_lower_SIDE_left_edge = cube[3][4][1];
    let L_lower_SIDE_middle_edge = cube[2][4][1];
    let L_lower_SIDE_right_edge = cube[1][4][1];

    let L_right_SIDE_upper_edge = cube[0][1][1];
    let L_right_SIDE_middle_edge = cube[0][2][1];
    let L_right_SIDE_lower_edge = cube[0][3][1];
    
    // Perform the L move for the L face 

    // Rotate Corners Counter Clockwise on the L face
    cube[3][1][0] = L_FACE_upper_right_corner; // L_FACE_upper_left_corner <---- L_FACE_upper_right_corner
    cube[1][1][0] = L_FACE_lower_right_corner; // L_FACE_upper_right_corner <---- L_FACE_lower_right_corner
    cube[1][3][0] = L_FACE_lower_left_corner; // L_FACE_lower_right_corner <---- L_FACE_lower_left_corner
    cube[3][3][0] = L_FACE_upper_left_corner; // L_FACE_lower_left_corner <---- L_FACE_upper_left_corner

    
    // Rotate Edges Counter Clockwise on the L face
    cube[2][1][0] = L_FACE_right_edge; // L_FACE_upper_edge <---- L_FACE_right_edge
    cube[1][2][0] = L_FACE_lower_edge; // L_FACE_right_edge <---- L_FACE_lower_edge
    cube[2][3][0] = L_FACE_left_edge; // L_FACE_lower_edge <---- L_FACE_left_edge
    cube[3][2][0] = L_FACE_upper_edge; // L_FACE_left_edge <---- L_FACE_upper_edge

    // Rotate Connected Edges Counter Clockwise on the L face
    cube[3][0][1] = L_right_SIDE_upper_edge; // L_upper_SIDE_left_edge <---- L_right_SIDE_upper_edge
    cube[2][0][1] = L_right_SIDE_middle_edge; // L_upper_SIDE_middle_edge <---- L_right_SIDE_middle_edge
    cube[1][0][1] = L_right_SIDE_lower_edge; // L_upper_SIDE_right_edge <---- L_right_SIDE_lower_edge

    cube[4][1][1] = L_upper_SIDE_right_edge; // L_left_SIDE_upper_edge <---- L_upper_SIDE_right_edge
    cube[4][2][1] = L_upper_SIDE_middle_edge; // L_left_SIDE_middle_edge <---- L_upper_SIDE_middle_edge
    cube[4][3][1] = L_upper_SIDE_left_edge; // L_left_SIDE_lower_edge <---- L_upper_SIDE_left_edge

    cube[3][4][1] = L_left_SIDE_upper_edge; // L_lower_SIDE_left_edge <---- L_left_SIDE_upper_edge
    cube[2][4][1] = L_left_SIDE_middle_edge; // L_lower_SIDE_middle_edge <---- L_left_SIDE_middle_edge
    cube[1][4][1] = L_left_SIDE_lower_edge; // L_lower_SIDE_right_edge <---- L_left_SIDE_lower_edge

    cube[0][1][1] = L_lower_SIDE_right_edge; // L_right_SIDE_upper_edge <--- L_lower_SIDE_right_edge
    cube[0][2][1] = L_lower_SIDE_middle_edge; // L_right_SIDE_middle_edge <--- L_lower_SIDE_middle_edge
    cube[0][3][1] = L_lower_SIDE_left_edge; // L_right_SIDE_lower_edge <--- L_lower_SIDE_left_edge 
}

// This function performs the R move on the rubiks cube array
pub fn Perform_R(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the R face
    let R_FACE_upper_left_corner = cube[1][1][4];
    let R_FACE_upper_right_corner = cube[3][1][4];
    let R_FACE_lower_left_corner = cube[1][3][4];
    let R_FACE_lower_right_corner = cube[3][3][4];

    // // Get Edges of the R face
    let R_FACE_upper_edge = cube[2][1][4];
    let R_FACE_left_edge = cube[1][2][4];
    let R_FACE_lower_edge = cube[2][3][4];
    let R_FACE_right_edge = cube[3][2][4];

    // // Get the Connected Edges of the R face
    let R_upper_SIDE_left_edge = cube [1][0][3];
    let R_upper_SIDE_middle_edge = cube[2][0][3];
    let R_upper_SIDE_right_edge = cube[3][0][3];

    let R_left_SIDE_upper_edge = cube[0][1][3];
    let R_left_SIDE_middle_edge = cube[0][2][3];
    let R_left_SIDE_lower_edge = cube[0][3][3];


    let R_right_SIDE_upper_edge = cube[4][1][3]; 
    let R_right_SIDE_middle_edge = cube[4][2][3];
    let R_right_SIDE_lower_edge = cube[4][3][3];

    let R_lower_SIDE_left_edge = cube[1][4][3];
    let R_lower_SIDE_middle_edge = cube[2][4][3];
    let R_lower_SIDE_right_edge = cube[3][4][3];
    
    // Perform the R move for the R face 

    // Rotate Corners Clockwise on the R face
    cube[1][1][4] = R_FACE_lower_right_corner; // R_FACE_upper_left_corner <---- R_FACE_lower_right_corner
    cube[3][1][4] = R_FACE_upper_left_corner; //R_FACE_upper_right_corner <---- R_FACE_upper_left_corner
    cube[1][3][4] = R_FACE_lower_right_corner; // R_FACE_lower_left_corner <---- R_FACE_lower_right_corner
    cube[3][3][4] = R_FACE_upper_right_corner; // R_FACE_lower_right_corner  <---- R_FACE_upper_right_corner
    
    // Rotate Edges Clockwise on the R face
    cube[2][1][4] = R_FACE_left_edge; // R_FACE_upper_edge <---- R_FACE_left_edge
    cube[1][2][4] = R_FACE_lower_edge; // R_FACE_left_edge <---- R_FACE_lower_edge
    cube[2][3][4] = R_FACE_right_edge; // R_FACE_lower_edge <---- R_FACE_right_edge
    cube[3][2][4] = R_FACE_upper_edge; // R_FACE_right_edge <---- R_FACE_upper_edge

    // Rotate Connected Edges Clockwise on the R face
    cube [1][0][3] = R_left_SIDE_lower_edge; // R_upper_SIDE_left_edge <---- R_left_SIDE_lower_edge
    cube[2][0][3] = R_left_SIDE_middle_edge; // R_upper_SIDE_middle_edge <---- R_left_SIDE_middle_edge
    cube[3][0][3] = R_left_SIDE_upper_edge; // R_upper_SIDE_right_edge <---- R_left_SIDE_upper_edge

    cube[0][1][3] = R_lower_SIDE_left_edge; // R_left_SIDE_upper_edge <---- R_lower_SIDE_left_edge
    cube[0][2][3] = R_lower_SIDE_middle_edge; // R_left_SIDE_middle_edge <---- R_lower_SIDE_middle_edge
    cube[0][3][3] = R_lower_SIDE_right_edge; // R_left_SIDE_lower_edge <---- R_lower_SIDE_right_edge


    cube[4][1][3] = R_upper_SIDE_left_edge; //R_right_SIDE_upper_edge <---- R_upper_SIDE_left_edge
    cube[4][2][3] = R_upper_SIDE_middle_edge; // R_right_SIDE_middle_edge <---- R_upper_SIDE_middle_edge
    cube[4][3][3] = R_upper_SIDE_right_edge; // R_right_SIDE_lower_edge <---- R_upper_SIDE_right_edge

    cube[1][4][3] = R_right_SIDE_lower_edge; // R_lower_SIDE_left_edge <---- R_right_SIDE_lower_edge
    cube[2][4][3] = R_right_SIDE_middle_edge; // R_lower_SIDE_middle_edge <---- R_right_SIDE_middle_edge
    cube[3][4][3] = R_right_SIDE_upper_edge; // R_lower_SIDE_right_edge <---- R_right_SIDE_upper_edge
}

// This function performs the R move on the rubiks cube array
pub fn Perform_R_Prime(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the R face
    let R_FACE_upper_left_corner = cube[1][1][4];
    let R_FACE_upper_right_corner = cube[3][1][4];
    let R_FACE_lower_left_corner = cube[1][3][4];
    let R_FACE_lower_right_corner = cube[3][3][4];

    // // Get Edges of the R face
    let R_FACE_upper_edge = cube[2][1][4];
    let R_FACE_left_edge = cube[1][2][4];
    let R_FACE_lower_edge = cube[2][3][4];
    let R_FACE_right_edge = cube[3][2][4];

    // // Get the Connected Edges of the R face
    let R_upper_SIDE_left_edge = cube [1][0][3];
    let R_upper_SIDE_middle_edge = cube[2][0][3];
    let R_upper_SIDE_right_edge = cube[3][0][3];

    let R_left_SIDE_upper_edge = cube[0][1][3];
    let R_left_SIDE_middle_edge = cube[0][2][3];
    let R_left_SIDE_lower_edge = cube[0][3][3];


    let R_right_SIDE_upper_edge = cube[4][1][3]; 
    let R_right_SIDE_middle_edge = cube[4][2][3];
    let R_right_SIDE_lower_edge = cube[4][3][3];

    let R_lower_SIDE_left_edge = cube[1][4][3];
    let R_lower_SIDE_middle_edge = cube[2][4][3];
    let R_lower_SIDE_right_edge = cube[3][4][3];
    
    // Perform the R move for the R face 

    // Rotate Corners Counter Clockwise on the R face
    cube[1][1][4] = R_FACE_upper_right_corner; // R_FACE_upper_left_corner <---- R_FACE_upper_right_corner
    cube[3][1][4] = R_FACE_lower_right_corner; // R_FACE_upper_right_corner <----  R_FACE_lower_right_corner
    cube[3][3][4] = R_FACE_lower_left_corner; // R_FACE_lower_right_corner <---- R_FACE_lower_left_corner
    cube[1][3][4] = R_FACE_upper_left_corner; // R_FACE_lower_left_corner <----  R_FACE_upper_left_corner
    
    // Rotate Edges Counter Clockwise on the R face
    cube[2][1][4] = R_FACE_right_edge; // R_FACE_upper_edge <---- R_FACE_right_edge
    cube[1][2][4] = R_FACE_upper_edge; // R_FACE_left_edge <----  R_FACE_upper_edge
    cube[2][3][4] = R_FACE_left_edge; // R_FACE_lower_edge <---- R_FACE_left_edge
    cube[3][2][4] = R_FACE_lower_edge; // R_FACE_right_edge <---- R_FACE_lower_edge

    // Rotate Connected Edges Counter Clockwise on the R face
    cube [1][0][3] = R_right_SIDE_upper_edge; // R_upper_SIDE_left_edge <---- R_right_SIDE_upper_edge
    cube[2][0][3] = R_right_SIDE_middle_edge; // R_upper_SIDE_middle_edge <---- R_right_SIDE_middle_edge
    cube[3][0][3] = R_right_SIDE_lower_edge; // R_upper_SIDE_right_edge <---- R_right_SIDE_lower_edge

    cube[0][1][3] = R_upper_SIDE_right_edge; // R_left_SIDE_upper_edge <---- R_upper_SIDE_right_edge 
    cube[0][2][3] = R_upper_SIDE_middle_edge; // R_left_SIDE_middle_edge <---- R_upper_SIDE_middle_edge
    cube[0][3][3] = R_upper_SIDE_left_edge; // R_left_SIDE_lower_edge <---- R_upper_SIDE_left_edge


    cube[4][1][3] = R_lower_SIDE_right_edge; //R_right_SIDE_upper_edge <---- R_lower_SIDE_right_edge
    cube[4][2][3] = R_lower_SIDE_middle_edge; // R_right_SIDE_middle_edge <---- R_lower_SIDE_middle_edge
    cube[4][3][3] = R_lower_SIDE_left_edge; // R_right_SIDE_lower_edge <---- R_lower_SIDE_left_edge 

    cube[1][4][3] = R_left_SIDE_upper_edge; // R_lower_SIDE_left_edge <---- R_left_SIDE_upper_edge
    cube[2][4][3] = R_left_SIDE_middle_edge; // R_lower_SIDE_middle_edge <---- R_left_SIDE_middle_edge
    cube[3][4][3] = R_left_SIDE_lower_edge; // R_lower_SIDE_right_edge <---- R_left_SIDE_lower_edge 
}

// This function performs the F move on the rubiks cube array
pub fn Perform_F(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the F face
    let F_FACE_upper_left_corner = cube[0][1][1];
    let F_FACE_upper_right_corner = cube[0][1][3];
    let F_FACE_lower_left_corner = cube[0][3][1];
    let F_FACE_lower_right_corner = cube[0][3][3];

    // Get Edges of the F face
    let F_FACE_upper_edge = cube[0][1][2];
    let F_FACE_left_edge = cube[0][2][1];
    let F_FACE_lower_edge = cube[0][3][2];
    let F_FACE_right_edge = cube[0][2][3];

    // Get the Connected Edges of the F face
    let F_upper_SIDE_left_edge = cube [1][0][1];
    let F_upper_SIDE_middle_edge = cube[1][0][2];
    let F_upper_SIDE_right_edge = cube[1][0][3];

    let F_left_SIDE_upper_edge = cube[1][1][0];
    let F_left_SIDE_middle_edge = cube[1][2][0];
    let F_left_SIDE_lower_edge = cube[1][3][0];

    let F_right_SIDE_upper_edge = cube[1][1][4]; 
    let F_right_SIDE_middle_edge = cube[1][2][4];
    let F_right_SIDE_lower_edge =  cube[1][3][4];

    let F_lower_SIDE_left_edge = cube[1][4][1];
    let F_lower_SIDE_middle_edge = cube[1][4][2];
    let F_lower_SIDE_right_edge = cube[1][4][3];
    
    // Perform the F move for the F face 

    // Rotate Corners Counter Clockwise on the U face
    cube[0][1][1] = F_FACE_lower_left_corner; // F_FACE_upper_left_corner <---- F_FACE_lower_left_corner
    cube[0][1][3] = F_FACE_upper_left_corner; // F_FACE_upper_right_corner  <---- F_FACE_upper_left_corner
    cube[0][3][3] = F_FACE_upper_right_corner; //U_FACE_lower_right_corner <---- F_FACE_upper_right_corner
    cube[0][3][1] = F_FACE_lower_right_corner; // U_FACE_lower_left_corner <---- F_FACE_lower_right_corner
    
    // // Rotate Edges Counter Clockwise on the U face
    cube[0][1][2] = F_FACE_left_edge; // F_FACE_upper_edge <---- F_FACE_left_edge
    cube[0][2][1] = F_FACE_lower_edge; // F_FACE_left_edge <---- F_FACE_lower_edge
    cube[0][3][2] = F_FACE_right_edge; // F_FACE_lower_edge <---- F_FACE_right_edge
    cube[0][2][3] = F_FACE_upper_edge; // F_FACE_right_edge <---- F_FACE_upper_edge
    


    // // Rotate Connected Edges Counter Clockwise on the U face
    cube [1][0][1] = F_left_SIDE_lower_edge; // F_upper_SIDE_left_edge <---- F_left_SIDE_lower_edge
    cube[1][0][2] = F_left_SIDE_middle_edge; // F_upper_SIDE_middle_edge <---- F_left_SIDE_middle_edge
    cube[1][0][3] = F_left_SIDE_upper_edge; // F_upper_SIDE_right_edge  <---- F_left_SIDE_upper_edge

    cube[1][1][0] = F_lower_SIDE_left_edge; // F_left_SIDE_upper_edge <---- F_lower_SIDE_left_edge
    cube[1][2][0] = F_lower_SIDE_middle_edge; // F_left_SIDE_middle_edge <---- F_lower_SIDE_middle_edge
    cube[1][3][0] = F_lower_SIDE_right_edge; // F_left_SIDE_lower_edge <---- F_lower_SIDE_right_edge

    cube[1][1][4] = F_upper_SIDE_left_edge;  // F_right_SIDE_upper_edge <---- F_upper_SIDE_left_edge
    cube[1][2][4] = F_upper_SIDE_middle_edge; // F_right_SIDE_middle_edge <---- F_upper_SIDE_middle_edge
    cube[1][3][4] = F_upper_SIDE_right_edge; // F_right_SIDE_lower_edge   <---- F_upper_SIDE_right_edge

    cube[1][4][1] = F_right_SIDE_lower_edge; // F_lower_SIDE_left_edge <---- F_right_SIDE_lower_edge
    cube[1][4][2] = F_right_SIDE_middle_edge; // F_lower_SIDE_middle_edge <---- F_right_SIDE_middle_edge
    cube[1][4][3] = F_right_SIDE_upper_edge; // F_lower_SIDE_right_edge <---- F_right_SIDE_upper_edge
}

// This function performs the F' move on the rubiks cube array
pub fn Perform_F_Prime(cube: &mut [[[i32; 5]; 5]; 5]){

    // Get Corners of the F face
    let F_FACE_upper_left_corner = cube[0][1][1];
    let F_FACE_upper_right_corner = cube[0][1][3];
    let F_FACE_lower_left_corner = cube[0][3][1];
    let F_FACE_lower_right_corner = cube[0][3][3];

    // Get Edges of the F face
    let F_FACE_upper_edge = cube[0][1][2];
    let F_FACE_left_edge = cube[0][2][1];
    let F_FACE_lower_edge = cube[0][3][2];
    let F_FACE_right_edge = cube[0][2][3];

    // Get the Connected Edges of the F face
    let F_upper_SIDE_left_edge = cube [1][0][1];
    let F_upper_SIDE_middle_edge = cube[1][0][2];
    let F_upper_SIDE_right_edge = cube[1][0][3];

    let F_left_SIDE_upper_edge = cube[1][1][0];
    let F_left_SIDE_middle_edge = cube[1][2][0];
    let F_left_SIDE_lower_edge = cube[1][3][0];

    let F_right_SIDE_upper_edge = cube[1][1][4]; 
    let F_right_SIDE_middle_edge = cube[1][2][4];
    let F_right_SIDE_lower_edge =  cube[1][3][4];

    let F_lower_SIDE_left_edge = cube[1][4][1];
    let F_lower_SIDE_middle_edge = cube[1][4][2];
    let F_lower_SIDE_right_edge = cube[1][4][3];
    
    // Perform the F move for the F face 

    // Rotate Corners Counter Clockwise on the U face
    cube[0][1][1] = F_FACE_upper_right_corner; // F_FACE_upper_left_corner <---- F_FACE_upper_right_corner
    cube[0][1][3] = F_FACE_lower_right_corner; // F_FACE_upper_right_corner  <---- F_FACE_lower_right_corner
    cube[0][3][3] = F_FACE_lower_left_corner; //U_FACE_lower_right_corner <---- F_FACE_lower_left_corner
    cube[0][3][1] = F_FACE_upper_left_corner; // U_FACE_lower_left_corner <---- F_FACE_upper_left_corner
    
    // // // Rotate Edges Counter Clockwise on the U face
    cube[0][1][2] = F_FACE_right_edge; // F_FACE_upper_edge <---- F_FACE_right_edge
    cube[0][2][1] = F_FACE_upper_edge; // F_FACE_left_edge <---- F_FACE_upper_edge
    cube[0][3][2] = F_FACE_left_edge; // F_FACE_lower_edge <---- F_FACE_left_edge
    cube[0][2][3] = F_FACE_lower_edge; // F_FACE_right_edge <---- F_FACE_lower_edge
    
    // // // Rotate Connected Edges Counter Clockwise on the U face
    cube[1][0][1] = F_right_SIDE_upper_edge; // F_upper_SIDE_left_edge <---- F_right_SIDE_upper_edge
    cube[1][0][2] = F_right_SIDE_middle_edge; // F_upper_SIDE_middle_edge <---- F_right_SIDE_middle_edge
    cube[1][0][3] = F_right_SIDE_lower_edge; // F_upper_SIDE_right_edge  <---- F_right_SIDE_lower_edge

    cube[1][1][0] = F_upper_SIDE_right_edge; // F_left_SIDE_upper_edge <---- F_upper_SIDE_right_edge
    cube[1][2][0] = F_upper_SIDE_middle_edge; // F_left_SIDE_middle_edge <---- F_upper_SIDE_middle_edge
    cube[1][3][0] = F_upper_SIDE_left_edge; // F_left_SIDE_lower_edge <----  F_upper_SIDE_left_edge

    cube[1][1][4] = F_lower_SIDE_right_edge;  // F_right_SIDE_upper_edge <---- F_lower_SIDE_right_edge
    cube[1][2][4] = F_lower_SIDE_middle_edge; // F_right_SIDE_middle_edge <---- F_lower_SIDE_middle_edge
    cube[1][3][4] = F_lower_SIDE_left_edge; // F_right_SIDE_lower_edge   <---- F_lower_SIDE_left_edge 

    cube[1][4][1] = F_left_SIDE_upper_edge; // F_lower_SIDE_left_edge <---- F_left_SIDE_upper_edge
    cube[1][4][2] = F_left_SIDE_middle_edge; // F_lower_SIDE_middle_edge <---- F_left_SIDE_middle_edge
    cube[1][4][3] = F_left_SIDE_lower_edge; // F_lower_SIDE_right_edge <---- F_left_SIDE_lower_edge
 

}

// This function performs the U move on the rubiks cube array
pub fn Perform_U(cube: &mut [[[i32; 5]; 5]; 5]) {
    
    // Get Corners of the U face
    let U_FACE_back_left_corner = cube[3][0][1];
    let U_FACE_back_right_corner = cube[3][0][3];
    let U_FACE_front_left_corner = cube[1][0][1];
    let U_FACE_front_right_corner = cube[1][0][3];

    // Get Edges of the U face
    let U_FACE_back_edge = cube[3][0][2];
    let U_FACE_left_edge = cube[2][0][1];
    let U_FACE_front_edge = cube[1][0][2];
    let U_FACE_right_edge = cube[2][0][3];

    // // Get the Connected Edges of the U face
    let U_front_SIDE_left_edge = cube [0][1][1];
    let U_front_SIDE_middle_edge = cube [0][1][2];
    let U_front_SIDE_right_edge = cube [0][1][3];

    let U_back_SIDE_left_edge = cube [4][1][1];
    let U_back_SIDE_middle_edge = cube [4][1][2];
    let U_back_SIDE_right_edge = cube [4][1][3];

    let U_left_SIDE_left_edge = cube[1][1][0];
    let U_left_SIDE_middle_edge = cube[2][1][0];
    let U_left_SIDE_right_edge = cube[3][1][0];

    let U_right_SIDE_left_edge = cube[1][1][4];
    let U_right_SIDE_middle_edge = cube[2][1][4];
    let U_right_SIDE_right_edge = cube[3][1][4];
    
    // Perform the U move for the U face. The way the below code works is that it 
    // "places" the value at the location on the right into the location on the left.

    // Rotate Corners Clockwise on the U face
    cube[3][0][1] = U_FACE_front_left_corner;  // U_FACE_back_left_corner <---- U_FACE_front_left_corner
    cube[3][0][3] = U_FACE_front_right_corner; // U_FACE_back_right_corner <---- U_FACE_back_left_corner
    cube[1][0][3] = U_FACE_back_right_corner; // U_FACE_front_right_corner <---- U_FACE_back_right_corner
    cube[1][0][1] = U_FACE_front_right_corner; // U_FACE_front_left_corner  <---- U_FACE_front_right_corner


    // Rotate Edges on the U face
    cube[3][0][2] = U_FACE_right_edge; // U_FACE_back_edge <---- U_FACE_left_edge
    cube[2][0][3] = U_FACE_front_edge; // U_FACE_right_edge <---- U_FACE_back_edge
    cube[1][0][2] = U_FACE_left_edge; // U_FACE_front_edge <---- U_FACE_right_edge
    cube[2][0][1] = U_FACE_back_edge; // U_FACE_left_edge <---- U_FACE_front_edge

    // Rotate Connected Edges on the U face

    cube [0][1][1] = U_right_SIDE_left_edge; // U_front_SIDE_left_edge <---- U_right_SIDE_left_edge 
    cube [0][1][2] = U_right_SIDE_middle_edge; // U_front_SIDE_middle_edge <---- U_right_SIDE_middle_edge
    cube [0][1][3] = U_right_SIDE_right_edge; // U_front_SIDE_right_edge <---- U_right_SIDE_right_edge

    cube[1][1][0] = U_front_SIDE_left_edge; // U_left_SIDE_left_edge <---- U_front_SIDE_left_edge
    cube[2][1][0] = U_front_SIDE_middle_edge; // U_left_SIDE_middle_edge <---- U_front_SIDE_middle_edge
    cube[3][1][0] = U_front_SIDE_right_edge; // U_left_SIDE_middle_edge <---- U_front_SIDE_right_edge

    cube [4][1][1] = U_left_SIDE_left_edge; // U_back_SIDE_left_edge <---- U_left_SIDE_left_edge
    cube [4][1][2] = U_left_SIDE_middle_edge; // U_back_SIDE_middle_edge <---- U_left_SIDE_middle_edge
    cube [4][1][3] = U_left_SIDE_right_edge; // U_back_SIDE_right_edge <---- U_left_SIDE_right_edge
    
    cube[1][1][4] = U_back_SIDE_left_edge; // U_right_SIDE_left_edge <---- U_back_SIDE_left_edge
    cube[2][1][4] = U_back_SIDE_middle_edge; // U_right_SIDE_middle_edge  <---- U_back_SIDE_middle_edge
    cube[3][1][4] = U_back_SIDE_right_edge; // U_right_SIDE_right_edge <---- U_back_SIDE_right_edge
}

// This function performs the U' move on the rubiks cube array
pub fn Perform_U_Prime(cube: &mut [[[i32; 5]; 5]; 5]) {
    
    // Get Corners of the U face
    let U_FACE_back_left_corner = cube[3][0][1];
    let U_FACE_back_right_corner = cube[3][0][3];
    let U_FACE_front_left_corner = cube[1][0][1];
    let U_FACE_front_right_corner = cube[1][0][3];

    // Get Edges of the U face
    let U_FACE_back_edge = cube[3][0][2];
    let U_FACE_left_edge = cube[2][0][1];
    let U_FACE_front_edge = cube[1][0][2];
    let U_FACE_right_edge = cube[2][0][3];

    // // Get the Connected Edges of the U face
    let U_front_SIDE_left_edge = cube [0][1][1];
    let U_front_SIDE_middle_edge = cube [0][1][2];
    let U_front_SIDE_right_edge = cube [0][1][3];

    let U_back_SIDE_left_edge = cube [4][1][1];
    let U_back_SIDE_middle_edge = cube [4][1][2];
    let U_back_SIDE_right_edge = cube [4][1][3];

    let U_left_SIDE_left_edge = cube[1][1][0];
    let U_left_SIDE_middle_edge = cube[2][1][0];
    let U_left_SIDE_right_edge = cube[3][1][0];

    let U_right_SIDE_left_edge = cube[1][1][4];
    let U_right_SIDE_middle_edge = cube[2][1][4];
    let U_right_SIDE_right_edge = cube[3][1][4];
    
    // Perform the U move for the U face 

    // Rotate Corners Counter Clockwise on the U face
    cube[3][0][1] = U_FACE_back_right_corner;  // U_FACE_back_left_corner <---- U_FACE_back_right_corner
    cube[3][0][3] = U_FACE_front_right_corner; // U_FACE_back_right_corner <---- U_FACE_front_right_corner
    cube[1][0][1] = U_FACE_back_left_corner; // U_FACE_front_left_corner  <---- U_FACE_back_left_corner
    cube[1][0][3] = U_FACE_front_left_corner; // U_FACE_front_right_corner <---- U_FACE_front_left_corner

    // Rotate Edges Counter Clockwise on the U face
    cube[3][0][2] = U_FACE_right_edge; // U_FACE_back_edge <---- U_FACE_right_edge
    cube[2][0][3] = U_FACE_front_edge; // U_FACE_right_edge <---- U_FACE_front_edge
    cube[1][0][2] = U_FACE_left_edge; // U_FACE_front_edge <---- U_FACE_left_edge
    cube[2][0][1] = U_FACE_back_edge; // U_FACE_left_edge <---- U_FACE_back_edge

    // Rotate Connected Edges Counter Clockwise on the U face

    cube [0][1][1] = U_left_SIDE_left_edge; // U_front_SIDE_left_edge <---- U_left_SIDE_left_edge
    cube [0][1][2] = U_left_SIDE_middle_edge; // U_front_SIDE_middle_edge <---- U_left_SIDE_middle_edge
    cube [0][1][3] = U_left_SIDE_right_edge; // U_front_SIDE_right_edge <---- U_left_SIDE_right_edge

    cube[1][1][0] = U_back_SIDE_left_edge; // U_left_SIDE_left_edge <---- U_back_SIDE_left_edge
    cube[2][1][0] = U_back_SIDE_middle_edge; // U_left_SIDE_middle_edge <---- U_back_SIDE_middle_edge
    cube[3][1][0] = U_back_SIDE_right_edge; // U_left_SIDE_middle_edge <---- U_back_SIDE_right_edge


    cube [4][1][1] = U_right_SIDE_left_edge; // U_back_SIDE_left_edge <---- U_right_SIDE_left_edge
    cube [4][1][2] = U_right_SIDE_middle_edge; // U_back_SIDE_middle_edge <---- U_right_SIDE_middle_edge
    cube [4][1][3] = U_right_SIDE_right_edge; // U_back_SIDE_right_edge <---- U_right_SIDE_right_edge
    
    cube[1][1][4] = U_front_SIDE_left_edge; // U_right_SIDE_left_edge <---- U_front_SIDE_left_edge
    cube[2][1][4] = U_front_SIDE_middle_edge; // U_right_SIDE_middle_edge  <---- U_front_SIDE_middle_edge
    cube[3][1][4] = U_front_SIDE_right_edge; // U_right_SIDE_right_edge <---- U_front_SIDE_right_edge
}


