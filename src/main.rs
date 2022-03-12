// Using color initials for code to make it more readable, but storing
// colors as ints in tuples
const W: u8 = 0;
const Y: u8 = 1;
const R: u8 = 2;
const O: u8 = 3;
const B: u8 = 4;
const G: u8 = 5;

#[derive(Debug, Copy, Clone)]
struct Piece {
    // (x, y, z) coordinates of the pieces. So the corner at (0, 0, 0) is
    // touching the origin point, and (0, 0, 1) is the piece above.
    position: [u8; 3],

    // The visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: [u8; 3],
}

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
struct Face {
    axis: usize,
    value: u8,
}

#[derive(Debug, Clone, Copy)]
struct PocketCube {
    pieces: [Piece; 8],
}

impl PocketCube {
    // Returns a cube with white on the bottom, green on the back, and orange
    // on the left.
    fn get_solved_cube() -> PocketCube {
        return PocketCube {
            pieces: [
                Piece {
                    position: [0, 0, 0],
                    orientation: [W, G, O],
                },
                Piece {
                    position: [0, 0, 1],
                    orientation: [Y, G, O],
                },
                Piece {
                    position: [0, 1, 0],
                    orientation: [W, B, O],
                },
                Piece {
                    position: [0, 1, 1],
                    orientation: [Y, B, O],
                },
                Piece {
                    position: [1, 0, 0],
                    orientation: [W, G, R],
                },
                Piece {
                    position: [1, 0, 1],
                    orientation: [Y, G, R],
                },
                Piece {
                    position: [1, 1, 0],
                    orientation: [W, B, R],
                },
                Piece {
                    position: [1, 1, 1],
                    orientation: [Y, B, R],
                },
            ],
        };
    }

    // rotations are clockwise per face
    // face 0,0, 000 > 001 > 011 > 010 > 000 (11,10,00,01) (from left)
    // face 0,1, 100 > 110 > 111 > 101 > 100 (11,01,00,10) (from right)
    // 0/1 flip

    // face 1,0, 000 > 100 > 101 > 001 > 000 (11,01,00,10) (from back)
    // face 1,1, 010 > 011 > 111 > 110 > 010 (11,10,00,01) (from front)
    // 0/2 flip

    // face 2,0, 110 > 100 > 000 > 010 > 110 (11,10,00,01) (from bottom)
    // face 2,1, 111 > 011 > 001 > 101 > 111 (11,01,00,10) (from top)
    // 1/2 flip
    fn rotate(cube: &PocketCube, face: &Face) -> PocketCube {
        let mut new_cube = cube.clone();
        for piece in new_cube.pieces.iter_mut() {
            if piece.position[face.axis] == face.value {
                // Update position
                if (face.axis == 0 && face.value == 0)
                    || (face.axis == 1 && face.value == 1)
                    || (face.axis == 2 && face.value == 0)
                {
                    // 11 -> 10
                } else {
                    // 11 -> 01
                }

                // Update orientation
                match face.axis {
                    0 => {
                        let temp = piece.orientation[0];
                        piece.orientation[0] = piece.orientation[1];
                        piece.orientation[1] = temp;
                    }
                    1 => {
                        let temp = piece.orientation[0];
                        piece.orientation[0] = piece.orientation[2];
                        piece.orientation[2] = temp;
                    }
                    _ => {
                        let temp = piece.orientation[1];
                        piece.orientation[1] = piece.orientation[2];
                        piece.orientation[2] = temp;
                    }
                }
            }
        }
        return new_cube;
    }

    // Will perform n random rotations on cube and return.
    fn scramble(cube: PocketCube, n: u8) -> PocketCube {
        return cube.clone();
    }

    fn is_solved(cube: PocketCube) -> bool {
        return false;
    }

    // Will brute force every move and return true if one solves it.
    fn can_solve_in(n: u8) -> bool {
        return false;
    }
}

fn main() {
    let solved_cube = PocketCube::get_solved_cube();
    println!("{:?}", solved_cube);
}
