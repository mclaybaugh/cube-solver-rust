mod pocket_cube;

use pocket_cube::PocketCube;

fn main() {
    let solved_cube = PocketCube::new();
    let (scrambled, moves) = solved_cube.scramble(1);
    println!("scramble moves: {:?}", moves);

    let result = scrambled.maybe_solve_in(5);
    println!("maybe solve: {:?}", result);
}
