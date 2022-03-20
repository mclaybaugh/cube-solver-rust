use rand::Rng;

// initial Cube stuff
#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

fn opposite_color(c: Color) -> Color {
    match c {
        Color::White => Color::Yellow,
        Color::Yellow => Color::White,
        Color::Red => Color::Orange,
        Color::Orange => Color::Red,
        Color::Blue => Color::Green,
        Color::Green => Color::Blue,
    }
}

// TODO Also keep a history of rotations done on a cube
#[derive(Debug, Copy, Clone)]
pub struct Corner {
    // (x, y, z) coordinates of the pieces. So the corner at (0, 0, 0) is
    // touching the origin point, and (0, 0, 1) is the piece above.
    position: [u8; 3],

    // The visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: [Color; 3],
}

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
#[derive(Debug, Clone, Copy)]
pub struct Face {
    axis: usize,
    value: u8,
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

fn random_face() -> Face {
    let x: u8 = rand::thread_rng().gen_range(0..6);
    get_face(x)
}

fn rand_face_index() -> u8 {
    rand::thread_rng().gen_range(0..6)
}

// for the x/y face (0 axis), the determinant value is z (pos index 2)
fn face_colors_by_corner(corner: &Corner) -> [(Face, Color); 6] {
    [
        (
            Face {
                axis: 0,
                value: corner.position[2],
            },
            corner.orientation[0],
        ),
        (
            Face {
                axis: 0,
                value: opposite_pos(corner.position[2]),
            },
            opposite_color(corner.orientation[0]),
        ),
        (
            Face {
                axis: 1,
                value: corner.position[1],
            },
            corner.orientation[1],
        ),
        (
            Face {
                axis: 1,
                value: opposite_pos(corner.position[1]),
            },
            opposite_color(corner.orientation[1]),
        ),
        (
            Face {
                axis: 2,
                value: corner.position[0],
            },
            corner.orientation[2],
        ),
        (
            Face {
                axis: 2,
                value: opposite_pos(corner.position[0]),
            },
            opposite_color(corner.orientation[2]),
        ),
    ]
}

fn face_color_in_list(face_color: (Face, Color), list: [(Face, Color); 6]) -> bool {
    let (face, col) = face_color;
    for x in list {
        let (lface, lcol) = x;
        // if face matches then ensure color matches
        if face.axis == lface.axis && face.value == lface.value {
            if col != lcol {
                return false;
            }
        }
    }
    return true;
}

fn opposite_pos(x: u8) -> u8 {
    match x {
        0 => 1,
        _ => 0,
    }
}

// Indexes of arrays of size 3 are always 0,1,2
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
    corners: [Corner; 8],
}

impl PocketCube {
    // Returns a cube with white on the bottom, green on the back, and orange
    // on the left.
    pub fn get_solved_cube() -> PocketCube {
        return PocketCube {
            corners: [
                Corner {
                    position: [0, 0, 0],
                    orientation: [Color::White, Color::Green, Color::Orange],
                },
                Corner {
                    position: [0, 0, 1],
                    orientation: [Color::Yellow, Color::Green, Color::Orange],
                },
                Corner {
                    position: [0, 1, 0],
                    orientation: [Color::White, Color::Blue, Color::Orange],
                },
                Corner {
                    position: [0, 1, 1],
                    orientation: [Color::Yellow, Color::Blue, Color::Orange],
                },
                Corner {
                    position: [1, 0, 0],
                    orientation: [Color::White, Color::Green, Color::Red],
                },
                Corner {
                    position: [1, 0, 1],
                    orientation: [Color::Yellow, Color::Green, Color::Red],
                },
                Corner {
                    position: [1, 1, 0],
                    orientation: [Color::White, Color::Blue, Color::Red],
                },
                Corner {
                    position: [1, 1, 1],
                    orientation: [Color::Yellow, Color::Blue, Color::Red],
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
    pub fn rotate(cube: PocketCube, face_index: u8) -> PocketCube {
        let face = get_face(face_index);
        let mut new_cube = cube.clone();
        for piece in new_cube.corners.iter_mut() {
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
        return new_pos;
    }

    // Indexes of arrays of size 3 are always 0,1,2
    fn rotate_orientation(orientation: &mut [Color], axis: usize) -> [Color; 3] {
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
        let list = face_colors_by_corner(&cube.corners[0]);
        // for each remaining piece, ensure each side matches face
        for i in 1..6 {
            // get color/face pair, then check if in face_colors
            let corner_face_colors = [
                (
                    Face {
                        axis: 0,
                        value: cube.corners[i].position[2],
                    },
                    cube.corners[i].orientation[0],
                ),
                (
                    Face {
                        axis: 1,
                        value: cube.corners[i].position[1],
                    },
                    cube.corners[i].orientation[1],
                ),
                (
                    Face {
                        axis: 2,
                        value: cube.corners[i].position[0],
                    },
                    cube.corners[i].orientation[2],
                ),
            ];
            for face_color in corner_face_colors {
                if !face_color_in_list(face_color, list) {
                    return false;
                }
            }
        }
        return true;
    }

    // Perform n random rotations on cube and return.
    // TODO make this work on any type with trait of "puzzle_cube"
    pub fn scramble(mut cube: PocketCube, n: u8) -> (PocketCube, Vec<u8>) {
        let mut rotations: Vec<u8> = Vec::new();
        for _ in 0..n {
            let face_index = rand_face_index();
            rotations.push(face_index);
            cube = PocketCube::rotate(cube, face_index);
        }
        return (cube, rotations);
    }

    // Brute force every move and return true if one solves it.
    // iterative deepening depth-first traversal
    // needs to return breadcrumb tail on success, or false/None
    pub fn maybe_solve_in(cube: PocketCube, n: u8) -> (Option<Vec<u8>>, u64) {
        let mut depth = 0;
        let nodes_checked = 0;
        loop {
            let moves: Vec<u8> = Vec::new();
            let (returned_moves, is_solved, nodes_checked) =
                PocketCube::depth_limited_search(cube, &moves, depth, nodes_checked);
            if is_solved {
                return (Some(returned_moves.clone()), nodes_checked);
            }
            if depth == n {
                break;
            } else {
                depth = depth + 1;
            }
        }
        return (None, nodes_checked);
    }

    fn depth_limited_search(
        cube: PocketCube,
        moves: &Vec<u8>,
        n: u8,
        mut nodes_checked: u64,
    ) -> (Vec<u8>, bool, u64) {
        // end of line, check if solved
        if n == 0 {
            if PocketCube::is_solved(&cube) {
                return (moves.clone(), true, nodes_checked + 1);
            } else {
                return (moves.clone(), false, nodes_checked + 1);
            }
        }

        // do check on children (rotations)
        for i in 0..6 {
            // don't do same move 4 times in a row
            let move_len = moves.len();
            if move_len >= 3 {
                if moves[move_len - 1] == i && moves[move_len - 2] == i && moves[move_len - 3] == i
                {
                    continue;
                }
            }

            let mut new_moves = moves.clone();
            new_moves.push(i);
            let ncube = PocketCube::rotate(cube, i);
            let (new_moves, solved, nodes_checked_r) =
                PocketCube::depth_limited_search(ncube, &new_moves, n - 1, nodes_checked);
            nodes_checked = nodes_checked_r;
            if solved {
                return (new_moves.clone(), solved, nodes_checked);
            }
        }
        return (moves.clone(), false, nodes_checked);
    }
}
