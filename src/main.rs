mod cube;

use cube::maybe_solve_in;
use cube::scramble;
use cube::Pocket;

fn main() {
    let solved_cube = Pocket::new();
    let (scrambled, moves) = scramble(&solved_cube, 2);
    println!("scramble moves: {:?}", moves);

    let result = maybe_solve_in(&scrambled, 2);
    println!("maybe solve: {:?}", result);
}
