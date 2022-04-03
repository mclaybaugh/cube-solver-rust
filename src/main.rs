mod cube;

use cube::pocket::PocketCube;

fn main() {
    let solved_cube = PocketCube::new();
    println!("cube: {}", solved_cube);
    // let (scrambled, moves) = solved_cube.scramble(1);
    // println!("scramble moves: {:?}", moves);

    // let result = scrambled.maybe_solve_in(5);
    // println!("maybe solve: {:?}", result);
}
