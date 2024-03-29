mod color;
mod rotation;

use color::Color;
use rotation::Face;
use rotation::Rdir;
use rotation::Rotation;
use rotation::RotationDetails;
use std::fmt;

pub trait Cube {
    fn rotate(&self, r: Rotation) -> Self;
    fn is_solved(&self) -> bool;
}

// Perform n random rotations on cube and return.
// TODO make this work on any type with trait of "puzzle_cube"
pub fn scramble<T: Cube + Clone>(cube: &T, n: u8) -> (T, Vec<Rotation>) {
    let mut rotations: Vec<Rotation> = Vec::new();
    let mut new_cube = cube.clone();
    for _ in 0..n {
        let rotation_index = Rotation::random();
        rotations.push(rotation_index);
        new_cube = new_cube.rotate(rotation_index);
    }
    return (new_cube, rotations);
}

// Brute force every move and return true if one solves it.
// iterative deepening depth-first traversal
// needs to return breadcrumb tail on success, or false/None
pub fn brute_force<T: Cube + Clone>(cube: &T, n: u8) -> (Option<Vec<Rotation>>, u64) {
    // println!("max depth: {}", n);
    let mut depth = 0;
    let mut nodes_checked = 0;
    while depth <= n {
        let moves: Vec<Rotation> = Vec::new();
        let (returned_moves, is_solved, nodes_checked_r) =
            depth_limited_search(cube.clone(), &moves, depth, nodes_checked);
        nodes_checked = nodes_checked_r;
        if is_solved {
            return (Some(returned_moves.clone()), nodes_checked);
        }
        depth += 1;
    }
    return (None, nodes_checked);
}

fn depth_limited_search<T: Cube + Clone>(
    cube: T,
    moves: &Vec<Rotation>,
    n: u8,
    mut nodes_checked: u64,
) -> (Vec<Rotation>, bool, u64) {
    // end of line, check if solved
    if n == 0 {
        nodes_checked += 1;
        if cube.is_solved() {
            return (moves.clone(), true, nodes_checked);
        } else {
            return (moves.clone(), false, nodes_checked);
        }
    }

    // do check on children (rotations)
    for r in Rotation::pocket_array() {
        // println!("trying {:?}, {}", r, n);
        let move_len = moves.len();
        // don't reverse prior move
        if move_len > 0 && r.is_reverse(moves[move_len - 1]) {
            continue;
        }
        // don't do same move 4 times in a row
        if move_len > 3
            && (moves[move_len - 1] == r && moves[move_len - 2] == r && moves[move_len - 3] == r)
        {
            continue;
        }

        let mut new_moves = moves.clone();
        new_moves.push(r);
        let ncube = cube.rotate(r);
        let (new_moves, solved, nodes_checked_r) =
            depth_limited_search(ncube, &new_moves, n - 1, nodes_checked);
        nodes_checked = nodes_checked_r;
        if solved {
            return (new_moves.clone(), solved, nodes_checked);
        }
    }
    return (moves.clone(), false, nodes_checked);
}

#[derive(Debug, Copy, Clone)]
pub struct Corner {
    // (x, y, z) coordinates of the pieces. So the corner at (0, 0, 0) is
    // touching the origin point, and (0, 0, 1) is the piece above.
    position: [u8; 3],

    // The visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: [Color; 3],
}

impl PartialEq for Corner {
    fn eq(&self, other: &Self) -> bool {
        if self.position != other.position || self.orientation != other.orientation {
            return false;
        }

        return true;
    }
}

fn corner_from_array(xs: [u8; 6]) -> Corner {
    Corner {
        position: [xs[0], xs[1], xs[2]],
        orientation: [
            Color::from_num(xs[3]),
            Color::from_num(xs[4]),
            Color::from_num(xs[5]),
        ],
    }
}

// Indexes of arrays of size 3 are always 0,1,2
fn get_other_indexes(i: usize) -> (usize, usize) {
    match i {
        0 => (1, 2),
        1 => (0, 2),
        2 => (0, 1),
        _ => {
            panic!("get_other_indexes invalid index: {}", i);
        }
    }
}

pub fn opposite_pos(x: u8) -> u8 {
    match x {
        0 => 1,
        _ => 0,
    }
}

// for the x/y face (0 axis), the determinant value is z (pos index 2)
pub fn face_colors_by_corner(corner: &Corner) -> [(Face, Color); 6] {
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
            corner.orientation[0].opposite(),
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
            corner.orientation[1].opposite(),
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
            corner.orientation[2].opposite(),
        ),
    ]
}

pub fn face_color_in_list(face_color: (Face, Color), list: [(Face, Color); 6]) -> bool {
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

// Pocket cube stuff here
#[derive(Debug, Clone, Copy)]
pub struct Pocket {
    corners: [Corner; 8],
}

impl fmt::Display for Pocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, corner) in self.corners.iter().enumerate() {
            if i == 0 {
                write!(f, "\n(\n").unwrap();
            }
            write!(
                f,
                "{} {} {} {} {} {}",
                corner.position[0],
                corner.position[1],
                corner.position[2],
                corner.orientation[0].to_num(),
                corner.orientation[1].to_num(),
                corner.orientation[2].to_num()
            )
            .unwrap();
            if i == 7 {
                write!(f, "\n)\n").unwrap();
            } else {
                write!(f, "\n").unwrap();
            }
        }
        Ok(())
    }
}

impl PartialEq for Pocket {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..8 {
            if self.corners[i] != other.corners[i] {
                return false;
            }
        }
        return true;
    }
}

impl Pocket {
    fn to_array(&self) -> [[u8; 6]; 8] {
        let mut a: [[u8; 6]; 8] = [[0; 6]; 8];
        for (i, corner) in self.corners.iter().enumerate() {
            a[i] = [
                corner.position[0],
                corner.position[1],
                corner.position[2],
                corner.orientation[0].to_num(),
                corner.orientation[1].to_num(),
                corner.orientation[2].to_num(),
            ];
        }

        a
    }

    fn from_array(xs: [[u8; 6]; 8]) -> Pocket {
        Pocket {
            corners: [
                corner_from_array(xs[0]),
                corner_from_array(xs[1]),
                corner_from_array(xs[2]),
                corner_from_array(xs[3]),
                corner_from_array(xs[4]),
                corner_from_array(xs[5]),
                corner_from_array(xs[6]),
                corner_from_array(xs[7]),
            ],
        }
    }

    // Returns a cube with white on the bottom, green on the back, and orange
    // on the left.
    // w0, y1, r2, o3, b4, g5
    pub fn new() -> Pocket {
        Pocket::from_array([
            [0, 0, 0, 0, 5, 3],
            [0, 0, 1, 1, 5, 3],
            [0, 1, 0, 0, 4, 3],
            [0, 1, 1, 1, 4, 3],
            [1, 0, 0, 0, 5, 2],
            [1, 0, 1, 1, 5, 2],
            [1, 1, 0, 0, 4, 2],
            [1, 1, 1, 1, 4, 2],
        ])
    }

    fn rotate_position(pos: [u8; 3], rotation: &RotationDetails) -> [u8; 3] {
        let indexes: (usize, usize) = get_other_indexes(rotation.face.axis);
        let current_vals = (pos[indexes.0], pos[indexes.1]);
        let new_vals: (u8, u8);
        if (rotation.face.axis == 0
            && rotation.face.value == 0
            && rotation.direction == Rdir::Clockwise)
            || (rotation.face.axis == 0
                && rotation.face.value == 1
                && rotation.direction == Rdir::CounterClockwise)
            || (rotation.face.axis == 1
                && rotation.face.value == 1
                && rotation.direction == Rdir::Clockwise)
            || (rotation.face.axis == 1
                && rotation.face.value == 0
                && rotation.direction == Rdir::CounterClockwise)
            || (rotation.face.axis == 2
                && rotation.face.value == 0
                && rotation.direction == Rdir::Clockwise)
            || (rotation.face.axis == 2
                && rotation.face.value == 1
                && rotation.direction == Rdir::CounterClockwise)
        {
            new_vals = match current_vals {
                (1, 1) => (1, 0),
                (1, 0) => (0, 0),
                (0, 0) => (0, 1),
                (0, 1) => (1, 1),
                _ => {
                    panic!("rotate position match failed: {:?}", current_vals);
                }
            };
        } else {
            new_vals = match current_vals {
                (1, 1) => (0, 1),
                (0, 1) => (0, 0),
                (0, 0) => (1, 0),
                (1, 0) => (1, 1),
                _ => {
                    panic!("rotate position match failed: {:?}", current_vals);
                }
            };
        }
        let mut new_pos = [0; 3];
        new_pos[indexes.0] = new_vals.0;
        new_pos[indexes.1] = new_vals.1;
        new_pos[rotation.face.axis] = pos[rotation.face.axis];
        // println!("pos: {:?}, new: {:?}, indexes: {:?}", pos, new_pos, indexes);
        return new_pos;
    }

    // To update orientations, the color on the same side as the axis does not
    // change, and the other two swap.
    fn rotate_orientation(orientation: &[Color; 3], axis: usize) -> [Color; 3] {
        let mut n_orient = orientation.clone();
        // Indexes of arrays of size 3 are always 0,1,2
        match axis {
            0 => n_orient.swap(0, 1),
            1 => n_orient.swap(0, 2),
            _ => n_orient.swap(1, 2),
        };

        n_orient
    }
}

impl Cube for Pocket {
    // rotations are clockwise per face
    // face 0,0; 000 > 001 > 011 > 010 > 000 (11,10,00,01) (from left)
    // counterclockwise:
    // face 0,0; 000 > 010 > 011 > 001 > 000 (11,01,00,10)

    // face 0,1, 100 > 110 > 111 > 101 > 100 (11,01,00,10) (from right)
    // 0/1 flip

    // face 1,0, 000 > 100 > 101 > 001 > 000 (11,01,00,10) (from back)
    // face 1,1, 010 > 011 > 111 > 110 > 010 (11,10,00,01) (from front)
    // 0/2 flip

    // face 2,0, 110 > 100 > 000 > 010 > 110 (11,10,00,01) (from bottom)
    // face 2,1, 111 > 011 > 001 > 101 > 111 (11,01,00,10) (from top)
    // 1/2 flip
    fn rotate(&self, r: Rotation) -> Pocket {
        let rotation = r.details();
        let mut new_cube = self.clone();
        for piece in new_cube.corners.iter_mut() {
            if piece.position[rotation.face.axis] == rotation.face.value {
                piece.position = Pocket::rotate_position(piece.position, &rotation);
                piece.orientation =
                    Pocket::rotate_orientation(&mut piece.orientation, rotation.face.axis);
            }
        }
        return new_cube;
    }

    // from piece at pieces[0], we know all face colors because the three
    // on the piece and all the opposites are given
    fn is_solved(&self) -> bool {
        let list = face_colors_by_corner(&self.corners[0]);
        // for each remaining piece, ensure each side matches face
        for i in 1..8 {
            // get color/face pair, then check if in face_colors
            let corner_face_colors = [
                (
                    Face {
                        axis: 0,
                        value: self.corners[i].position[2],
                    },
                    self.corners[i].orientation[0],
                ),
                (
                    Face {
                        axis: 1,
                        value: self.corners[i].position[1],
                    },
                    self.corners[i].orientation[1],
                ),
                (
                    Face {
                        axis: 2,
                        value: self.corners[i].position[0],
                    },
                    self.corners[i].orientation[2],
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pocket_rotations_correct() {
        check_rotation(
            Rotation::L,
            [
                [0, 0, 1, 5, 0, 3],
                [0, 1, 1, 5, 1, 3],
                [0, 0, 0, 4, 0, 3],
                [0, 1, 0, 4, 1, 3],
                [1, 0, 0, 0, 5, 2],
                [1, 0, 1, 1, 5, 2],
                [1, 1, 0, 0, 4, 2],
                [1, 1, 1, 1, 4, 2],
            ],
        );
        check_rotation(
            Rotation::Lc,
            [
                [0, 1, 0, 5, 0, 3],
                [0, 0, 0, 5, 1, 3],
                [0, 1, 1, 4, 0, 3],
                [0, 0, 1, 4, 1, 3],
                [1, 0, 0, 0, 5, 2],
                [1, 0, 1, 1, 5, 2],
                [1, 1, 0, 0, 4, 2],
                [1, 1, 1, 1, 4, 2],
            ],
        );
        check_rotation(
            Rotation::R,
            [
                [0, 0, 0, 0, 5, 3],
                [0, 0, 1, 1, 5, 3],
                [0, 1, 0, 0, 4, 3],
                [0, 1, 1, 1, 4, 3],
                [1, 1, 0, 5, 0, 2],
                [1, 0, 0, 5, 1, 2],
                [1, 1, 1, 4, 0, 2],
                [1, 0, 1, 4, 1, 2],
            ],
        );
        check_rotation(
            Rotation::Bo,
            [
                [0, 1, 0, 0, 3, 5],
                [0, 0, 1, 1, 5, 3],
                [1, 1, 0, 0, 3, 4],
                [0, 1, 1, 1, 4, 3],
                [0, 0, 0, 0, 2, 5],
                [1, 0, 1, 1, 5, 2],
                [1, 0, 0, 0, 2, 4],
                [1, 1, 1, 1, 4, 2],
            ],
        );
    }

    fn check_rotation(r: Rotation, a: [[u8; 6]; 8]) -> () {
        let cube = Pocket::new().rotate(r);
        let ref_cube = Pocket::from_array(a);
        assert_eq!(cube, ref_cube, "cube: {}, ref: {}", cube, ref_cube)
    }

    #[test]
    fn pocket_brute_force_single_rotation() {
        for r in Rotation::pocket_array() {
            check_brute_force(r);
        }
    }

    fn check_brute_force(r: Rotation) -> () {
        let cube = Pocket::new().rotate(r);
        let (moves, c) = brute_force(&cube, 1);
        assert_eq!(
            moves,
            Some(vec!(r.reverse())),
            "r: {:?}, moves: {:?}, count: {}",
            r,
            moves,
            c
        )
    }
}
