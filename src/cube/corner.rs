mod color;

use color::Color;

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
        _ => (1, 2),
    }
}
