mod pocket_cube;

use pocket_cube::PocketCube;

fn main() {
    let solved_cube = PocketCube::new();
    let (scrambled, moves) = PocketCube::scramble(solved_cube, 1);
    println!("scramble moves: {:?}", moves);

    let result = PocketCube::maybe_solve_in(scrambled, 5);
    println!("maybe solve: {:?}", result);
}
