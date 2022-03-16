mod pocket_cube;

use pocket_cube as c;

fn main() {
    let solved_cube = c::PocketCube::get_solved_cube();
    let scrambled = c::PocketCube::scramble(solved_cube, 20);
    println!("{:?}", scrambled);
}
