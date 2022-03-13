use rand::Rng;

fn main() {
    let solved_cube = PocketCube::get_solved_cube();
    let scrambled = PocketCube::scramble(solved_cube, 20);
    println!("{:?}", scrambled);
}

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
#[derive(Debug, Clone, Copy)]
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
    fn rotate(cube: PocketCube, face: Face) -> PocketCube {
        let mut new_cube = cube.clone();
        for piece in new_cube.pieces.iter_mut() {
            if piece.position[face.axis] == face.value {
                piece.position = rotate_position(piece.position, &face);
                piece.orientation = rotate_orientation(piece.orientation, face.axis);
            }
        }
        return new_cube;
    }

    // Will perform n random rotations on cube and return.
    fn scramble(mut cube: PocketCube, n: u8) -> PocketCube {
        println!("I'm doing it!");
        for _ in 0..n {
            let x: u8 = rand::thread_rng().gen_range(0..6);
            let face = get_face(x);
            println!("{:?}", face);
            cube = PocketCube::rotate(cube, face);
        }
        return cube;
    }

    fn is_solved(cube: PocketCube) -> bool {
        return false;
    }

    // Will brute force every move and return true if one solves it.
    fn can_solve_in(n: u8) -> bool {
        return false;
    }
}

fn rotate_position(pos: [u8; 3], face: &Face) -> [u8; 3] {
    let indexes: (usize, usize) = get_other_indexes(face.axis);
    let v1 = pos[indexes.0];
    let v2 = pos[indexes.1];
    let new_vals: (u8, u8);
    if (face.axis == 0 && face.value == 0)
        || (face.axis == 1 && face.value == 1)
        || (face.axis == 2 && face.value == 0)
    {
        // 11 -> 10
        new_vals = match (v1, v2) {
            (1, 1) => (1, 0),
            (1, 0) => (0, 0),
            (0, 0) => (0, 1),
            (0, 1) => (1, 1),
            _ => (1, 1),
        };
    } else {
        // 11 -> 01
        new_vals = match (v1, v2) {
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

fn rotate_orientation(orientation: [u8; 3], axis: usize) -> [u8; 3] {
    match axis {
        0 => swap_3(orientation, 0, 1),
        1 => swap_3(orientation, 0, 2),
        _ => swap_3(orientation, 1, 2),
    }
}

fn get_other_indexes(i: usize) -> (usize, usize) {
    match i {
        0 => (1, 2),
        1 => (0, 2),
        _ => (1, 2),
    }
}

fn get_face(x: u8) -> Face {
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

fn swap_3(mut orientation: [u8; 3], i: usize, j: usize) -> [u8; 3] {
    let temp = orientation[i];
    orientation[i] = orientation[j];
    orientation[j] = temp;
    return orientation;
}
