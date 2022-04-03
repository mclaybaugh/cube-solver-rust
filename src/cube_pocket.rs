mod corner;

use corner::Corner;
use std::fmt;

// Pocket cube stuff here
#[derive(Debug, Clone, Copy)]
pub struct PocketCube {
    corners: [Corner; 8],
}

impl fmt::Display for PocketCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, corner) in self.corners.iter().enumerate() {
            if i == 0 {
                write!(f, "\n(\n");
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
            );
            if i == 7 {
                write!(f, "\n)\n");
            } else {
                write!(f, "\n");
            }
        }
        Ok(())
    }
}

impl PartialEq for PocketCube {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..8 {
            if self.corners[i] != other.corners[i] {
                return false;
            }
        }
        return true;
    }
}

impl PocketCube {
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

    fn from_array(xs: [[u8; 6]; 8]) -> PocketCube {
        PocketCube {
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
    pub fn new() -> PocketCube {
        PocketCube::from_array([
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
}

impl Cube for PocketCube {
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
    pub fn rotate(&self, rotation_index: u8) -> PocketCube {
        let rotation = get_rotation(rotation_index);
        let mut new_cube = self.clone();
        for piece in new_cube.corners.iter_mut() {
            if piece.position[rotation.face.axis] == rotation.face.value {
                piece.position = PocketCube::rotate_position(piece.position, &rotation);
                piece.orientation =
                    PocketCube::rotate_orientation(&mut piece.orientation, rotation.face.axis);
            }
        }
        return new_cube;
    }

    fn rotate_position(pos: [u8; 3], rotation: &Rotation) -> [u8; 3] {
        let indexes: (usize, usize) = get_other_indexes(rotation.face.axis);
        let current_vals = (pos[indexes.0], pos[indexes.1]);
        let new_vals: (u8, u8);
        if (rotation.face.axis == 0
            && rotation.face.value == 0
            && rotation.direction == RotationalDirection::Clockwise)
            || (rotation.face.axis == 0
                && rotation.face.value == 1
                && rotation.direction == RotationalDirection::CounterClockwise)
            || (rotation.face.axis == 1
                && rotation.face.value == 1
                && rotation.direction == RotationalDirection::Clockwise)
            || (rotation.face.axis == 1
                && rotation.face.value == 0
                && rotation.direction == RotationalDirection::CounterClockwise)
            || (rotation.face.axis == 2
                && rotation.face.value == 0
                && rotation.direction == RotationalDirection::Clockwise)
            || (rotation.face.axis == 2
                && rotation.face.value == 1
                && rotation.direction == RotationalDirection::CounterClockwise)
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
        new_pos[rotation.face.axis] = pos[rotation.face.axis];
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

    // from piece at pieces[0], we know all face colors because the three
    // on the piece and all the opposites are given
    pub fn is_solved(&self) -> bool {
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
    fn rotates_left_clockwise_correctly() {
        let cube = PocketCube::new().rotate(0);
        let ref_cube = PocketCube::from_array([
            [0, 0, 1, 5, 0, 3],
            [0, 1, 1, 5, 1, 3],
            [0, 0, 0, 4, 0, 3],
            [0, 1, 0, 4, 1, 3],
            [1, 0, 0, 0, 5, 2],
            [1, 0, 1, 1, 5, 2],
            [1, 1, 0, 0, 4, 2],
            [1, 1, 1, 1, 4, 2],
        ]);
        assert_eq!(cube, ref_cube, "cube: {}, ref: {}", cube, ref_cube);
    }

    fn rotates_1_correctly() {}
    fn rotates_2_correctly() {}
    fn rotates_3_correctly() {}
    fn rotates_4_correctly() {}
    fn rotates_5_correctly() {}
}
