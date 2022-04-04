mod cube;

use cube::brute_force;
use cube::scramble;
use cube::Pocket;

fn main() {
    let solved_cube = Pocket::new();
    let (scrambled, moves) = scramble(&solved_cube, 1);
    println!("scramble moves: {:?}", moves);

    let result = brute_force(&scrambled, 1);
    println!("maybe solve: {:?}", result);
}
