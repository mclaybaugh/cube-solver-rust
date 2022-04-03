mod cube;

use cube::*;

fn main() {
    let solved_cube = PocketCube::new();
    let (scrambled, moves) = scramble(&solved_cube, 1);
    println!("scramble moves: {:?}", moves);

    let result = maybe_solve_in(&scrambled, 1);
    println!("maybe solve: {:?}", result);
}
