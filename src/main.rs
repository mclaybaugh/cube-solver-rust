mod pocket_cube;

use pocket_cube as c;

fn main() {
    let solved_cube = c::PocketCube::get_solved_cube();
    let (scrambled, moves) = c::PocketCube::scramble(solved_cube, 1);
    println!("moves: {:?}", moves);

    let result = c::PocketCube::maybe_solve_in(scrambled, 5);
    println!("{:?}", result);
    // println!("trying 1");
    // let result = c::PocketCube::maybe_solve_in(scrambled, 1);
    // println!("{:?}", result);
    // println!("trying 2");
    // let result = c::PocketCube::maybe_solve_in(scrambled, 2);
    // println!("{:?}", result);
}
