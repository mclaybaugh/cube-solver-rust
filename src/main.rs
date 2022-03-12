// Using color initials for code to make it more readable, but storing
// colors as ints in tuples
const W: u8 = 0;
const Y: u8 = 1;
const R: u8 = 2;
const O: u8 = 3;
const B: u8 = 4;
const G: u8 = 5;

// First three ints are (x, y, z) coordinates of the pieces. So the corner at
// (0, 0, 0) is touching the origin point, and (0, 0, 1) is the piece right
// above that.
// Second three ints have the visible face colors of the corner in order
// by the side on the x/y plane, the x/z plane, and the y/z plane.
#[derive(Debug)]
struct Corner(u8, u8, u8, u8, u8, u8);

#[derive(Debug)]
struct PocketCube([Corner; 8]);

impl PocketCube {
    fn get_solved_cube() -> PocketCube {
        return PocketCube([
            Corner(0, 0, 0, W, G, O),
            Corner(0, 0, 1, Y, G, O),
            Corner(0, 1, 0, W, B, O),
            Corner(0, 1, 1, Y, B, O),
            Corner(1, 0, 0, W, G, R),
            Corner(1, 0, 1, Y, G, R),
            Corner(1, 1, 0, W, B, R),
            Corner(1, 1, 1, Y, B, R)
        ]);
    }
}

fn main() {
    let solved_cube = PocketCube::get_solved_cube();
    println!("{:?}", solved_cube);
}
