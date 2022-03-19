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

    println!("brute force it");
    println!("trying 0");
    let result = c::PocketCube::maybe_solve_in(scrambled, 0);
    println!("{:?}", result);
    println!("trying 1");
    let result = c::PocketCube::maybe_solve_in(scrambled, 1);
    println!("{:?}", result);
    println!("trying 2");
    let result = c::PocketCube::maybe_solve_in(scrambled, 2);
    println!("{:?}", result);
}
