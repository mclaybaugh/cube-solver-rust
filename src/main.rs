mod cube;

use cube::brute_force;
use cube::scramble;
use cube::Pocket;

fn main() {
    let solved_cube = Pocket::new();
    let (scrambled, moves) = scramble(&solved_cube, 10);
    println!("scramble: {:?}", moves);

    let result = brute_force(&scrambled, 6);
    println!("solution: {:?}", result);
}
