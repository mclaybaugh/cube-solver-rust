use rand::Rng;

// initial Cube stuff

// Using color initials for code to make it more readable, but storing
// colors as ints in tuples
pub const W: u8 = 0;
pub const Y: u8 = 1;
pub const R: u8 = 2;
pub const O: u8 = 3;
pub const B: u8 = 4;
pub const G: u8 = 5;

#[derive(Debug, Copy, Clone)]
pub struct Corner {
    // (x, y, z) coordinates of the pieces. So the corner at (0, 0, 0) is
    // touching the origin point, and (0, 0, 1) is the piece above.
    position: [u8; 3],

    // The visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: [u8; 3],
}

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
#[derive(Debug, Clone, Copy)]
pub struct Face {
    axis: usize,
    value: u8,
}

pub fn get_face(x: u8) -> Face {
    match x {
        0 => Face { axis: 0, value: 0 },
        1 => Face { axis: 0, value: 1 },
        2 => Face { axis: 1, value: 0 },
        3 => Face { axis: 1, value: 1 },
        4 => Face { axis: 2, value: 0 },
        5 => Face { axis: 2, value: 1 },
        _ => Face { axis: 0, value: 0 },
    }
}

// pub fn face_colors_by_piece(piece: &Corner) -> [(Face, u8); 6] {
//     // TODO finish this
// }

fn get_other_indexes(i: usize) -> (usize, usize) {
    match i {
        0 => (1, 2),
        1 => (0, 2),
        _ => (1, 2),
    }
}

// Pocket cube stuff here
#[derive(Debug, Clone, Copy)]
pub struct PocketCube {
    pieces: [Corner; 8],
}

impl PocketCube {
    // Returns a cube with white on the bottom, green on the back, and orange
    // on the left.
    pub fn get_solved_cube() -> PocketCube {
        return PocketCube {
            pieces: [
                Corner {
                    position: [0, 0, 0],
                    orientation: [W, G, O],
                },
                Corner {
                    position: [0, 0, 1],
                    orientation: [Y, G, O],
                },
                Corner {
                    position: [0, 1, 0],
                    orientation: [W, B, O],
                },
                Corner {
                    position: [0, 1, 1],
                    orientation: [Y, B, O],
                },
                Corner {
                    position: [1, 0, 0],
                    orientation: [W, G, R],
                },
                Corner {
                    position: [1, 0, 1],
                    orientation: [Y, G, R],
                },
                Corner {
                    position: [1, 1, 0],
                    orientation: [W, B, R],
                },
                Corner {
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
    pub fn rotate(cube: PocketCube, face: Face) -> PocketCube {
        let mut new_cube = cube.clone();
        for piece in new_cube.pieces.iter_mut() {
            if piece.position[face.axis] == face.value {
                piece.position = PocketCube::rotate_position(piece.position, &face);
                piece.orientation =
                    PocketCube::rotate_orientation(&mut piece.orientation, face.axis);
            }
        }
        return new_cube;
    }

    fn rotate_position(pos: [u8; 3], face: &Face) -> [u8; 3] {
        let indexes: (usize, usize) = get_other_indexes(face.axis);
        let current_vals = (pos[indexes.0], pos[indexes.1]);
        let new_vals: (u8, u8);
        if (face.axis == 0 && face.value == 0)
            || (face.axis == 1 && face.value == 1)
            || (face.axis == 2 && face.value == 0)
        {
            new_vals = match current_vals {
                (1, 1) => (1, 0),
                (1, 0) => (0, 0),
                (0, 0) => (0, 1),
                (0, 1) => (1, 1),
                _ => (1, 1),
            };
        } else {
            new_vals = match current_vals {
                (1, 1) => (0, 1),
                (0, 1) => (0, 0),
                (0, 0) => (1, 0),
                (1, 0) => (1, 1),
                _ => (1, 1),
            };
        }
        let mut new_pos = [0, 0, 0];
        new_pos[indexes.0] = new_vals.0;
        new_pos[indexes.1] = new_vals.1;
        new_pos[face.axis] = pos[face.axis];
        return pos;
    }

    fn rotate_orientation(orientation: &mut [u8], axis: usize) -> [u8; 3] {
        match axis {
            0 => orientation.swap(0, 1),
            1 => orientation.swap(0, 2),
            _ => orientation.swap(1, 2),
        };
        return orientation
            .try_into()
            .expect("orientation failed to convert to array");
    }

    // from piece at pieces[0], we know all face colors because the three
    // on the piece and all the opposites are given
    pub fn is_solved(cube: &PocketCube) -> bool {
        // get all face colors from piece
        // let faceColors: [(Face, u8); 6] = face_colors_by_piece(&cube.pieces[0]);
        // for each remaining piece, ensure each side matches face
        // on first failure, return false
        // else return true
        return false;
    }

    // Will perform n random rotations on cube and return.
    pub fn scramble(mut cube: PocketCube, n: u8) -> PocketCube {
        for _ in 0..n {
            let x: u8 = rand::thread_rng().gen_range(0..6);
            let face = get_face(x);
            println!("{:?}", face);
            cube = PocketCube::rotate(cube, face);
        }
        return cube;
    }

    // Will brute force every move and return true if one solves it.
    pub fn can_solve_in(n: u8) -> bool {
        return false;
    }
}
