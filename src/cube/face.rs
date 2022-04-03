mod color;

use color::Color;

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
#[derive(Debug, Clone, Copy)]
pub struct Face {
    axis: usize,
    value: u8,
}

fn opposite_pos(x: u8) -> u8 {
    match x {
        0 => 1,
        _ => 0,
    }
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
