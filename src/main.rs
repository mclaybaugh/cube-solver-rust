mod pocket_cube;

use pocket_cube as c;

fn main() {
    let solved_cube = c::PocketCube::get_solved_cube();
    println!("solved cube: {:?}", solved_cube);
    let is_solved = c::PocketCube::is_solved(&solved_cube);
    println!("Is solved?: {}", is_solved);
    let scrambled = c::PocketCube::scramble(solved_cube, 1);
    println!("scrambled cube: {:?}", scrambled);
    let is_solved = c::PocketCube::is_solved(&scrambled);
    println!("Is solved?: {}", is_solved);
}
