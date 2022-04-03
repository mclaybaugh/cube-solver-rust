mod face;

use face::Face;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
enum RotationalDirection {
    Clockwise,
    CounterClockwise,
}

struct Rotation {
    face: Face,
    direction: RotationalDirection,
}

fn get_rotation(x: u8) -> Rotation {
    match x {
        0 => Rotation {
            face: Face { axis: 0, value: 0 },
            direction: RotationalDirection::Clockwise,
        },
        1 => Rotation {
            face: Face { axis: 0, value: 0 },
            direction: RotationalDirection::CounterClockwise,
        },
        2 => Rotation {
            face: Face { axis: 0, value: 1 },
            direction: RotationalDirection::Clockwise,
        },
        3 => Rotation {
            face: Face { axis: 0, value: 1 },
            direction: RotationalDirection::CounterClockwise,
        },
        4 => Rotation {
            face: Face { axis: 1, value: 0 },
            direction: RotationalDirection::Clockwise,
        },
        5 => Rotation {
            face: Face { axis: 1, value: 0 },
            direction: RotationalDirection::CounterClockwise,
        },
        6 => Rotation {
            face: Face { axis: 1, value: 1 },
            direction: RotationalDirection::Clockwise,
        },
        7 => Rotation {
            face: Face { axis: 1, value: 1 },
            direction: RotationalDirection::CounterClockwise,
        },
        8 => Rotation {
            face: Face { axis: 2, value: 0 },
            direction: RotationalDirection::Clockwise,
        },
        9 => Rotation {
            face: Face { axis: 2, value: 0 },
            direction: RotationalDirection::CounterClockwise,
        },
        10 => Rotation {
            face: Face { axis: 2, value: 1 },
            direction: RotationalDirection::Clockwise,
        },
        11 => Rotation {
            face: Face { axis: 2, value: 1 },
            direction: RotationalDirection::CounterClockwise,
        },
        _ => {
            panic!("Invalid face index: {:?}", x);
        }
    }
}

fn is_reverse_rotation(i: u8, j: u8) -> bool {
    match (i, j) {
        (0, 1) | (1, 0) => true,
        (2, 3) | (3, 2) => true,
        (4, 5) | (5, 4) => true,
        (6, 7) | (7, 6) => true,
        (8, 9) | (9, 8) => true,
        (10, 11) | (11, 10) => true,
        _ => false,
    }
}

fn random_rotation_index() -> u8 {
    rand::thread_rng().gen_range(0..12)
}
