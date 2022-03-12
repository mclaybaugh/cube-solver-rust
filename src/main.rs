// Using color initials for code to make it more readable, but storing
// colors as ints in tuples
const W: u8 = 0;
const Y: u8 = 1;
const R: u8 = 2;
const O: u8 = 3;
const B: u8 = 4;
const G: u8 = 5;

#[derive(Debug)]
struct Piece {
    // (x, y, z) coordinates of the pieces. So the corner at (0, 0, 0) is
    // touching the origin point, and (0, 0, 1) is the piece above.
    position: (u8, u8, u8),

    // The visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: (u8, u8, u8),
}

#[derive(Debug)]
struct PocketCube([Piece; 8]);

impl PocketCube {
    fn get_solved_cube() -> PocketCube {
        return PocketCube([
            Piece {
                position: (0, 0, 0),
                orientation: (W, G, O),
            },
            Piece {
                position: (0, 0, 1),
                orientation: (Y, G, O),
            },
            Piece {
                position: (0, 1, 0),
                orientation: (W, B, O),
            },
            Piece {
                position: (0, 1, 1),
                orientation: (Y, B, O),
            },
            Piece {
                position: (1, 0, 0),
                orientation: (W, G, R),
            },
            Piece {
                position: (1, 0, 1),
                orientation: (Y, G, R),
            },
            Piece {
                position: (1, 1, 0),
                orientation: (W, B, R),
            },
            Piece {
                position: (1, 1, 1),
                orientation: (Y, B, R),
            },
        ]);
    }
}

fn main() {
    let solved_cube = PocketCube::get_solved_cube();
    println!("{:?}", solved_cube);
}
