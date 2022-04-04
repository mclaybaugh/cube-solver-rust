mod cube;

use cube::brute_force;
use cube::scramble;
use cube::Pocket;

fn main() {
    let solved_cube = Pocket::new();
    let (scrambled, moves) = scramble(&solved_cube, 20);
    println!("scramble: {:?}", moves);

    let result = brute_force(&scrambled, 8);
    println!("solution: {:?}", result);
}
