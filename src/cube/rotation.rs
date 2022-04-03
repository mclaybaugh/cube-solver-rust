use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rdir {
    Clockwise,
    CounterClockwise,
}

// Left clockwise, Left counter-clockwise, etc.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rotation {
    L,
    Lc,
    R,
    Rc,
    Ba,
    Bac,
    F,
    Fc,
    Bo,
    Boc,
    T,
    Tc,
}

impl Rotation {
    pub fn details(&self) -> RotationDetails {
        match self {
            L => RotationDetails {
                face: Face { axis: 0, value: 0 },
                direction: Rdir::Clockwise,
            },
            Lc => RotationDetails {
                face: Face { axis: 0, value: 0 },
                direction: Rdir::CounterClockwise,
            },
            R => RotationDetails {
                face: Face { axis: 0, value: 1 },
                direction: Rdir::Clockwise,
            },
            Rc => RotationDetails {
                face: Face { axis: 0, value: 1 },
                direction: Rdir::CounterClockwise,
            },
            Ba => RotationDetails {
                face: Face { axis: 1, value: 0 },
                direction: Rdir::Clockwise,
            },
            Bac => RotationDetails {
                face: Face { axis: 1, value: 0 },
                direction: Rdir::CounterClockwise,
            },
            F => RotationDetails {
                face: Face { axis: 1, value: 1 },
                direction: Rdir::Clockwise,
            },
            Fc => RotationDetails {
                face: Face { axis: 1, value: 1 },
                direction: Rdir::CounterClockwise,
            },
            Bo => RotationDetails {
                face: Face { axis: 2, value: 0 },
                direction: Rdir::Clockwise,
            },
            Boc => RotationDetails {
                face: Face { axis: 2, value: 0 },
                direction: Rdir::CounterClockwise,
            },
            T => RotationDetails {
                face: Face { axis: 2, value: 1 },
                direction: Rdir::Clockwise,
            },
            Tc => RotationDetails {
                face: Face { axis: 2, value: 1 },
                direction: Rdir::CounterClockwise,
            },
        }
    }

    pub fn random() -> Rotation {
        let i = rand::thread_rng().gen_range(0..12);
        Rotation::by_index(i)
    }

    pub fn by_index(i: u8) -> Rotation {
        match i {
            0 => Rotation::L,
            1 => Rotation::Lc,
            2 => Rotation::R,
            3 => Rotation::Rc,
            4 => Rotation::Ba,
            5 => Rotation::Bac,
            6 => Rotation::F,
            7 => Rotation::Fc,
            8 => Rotation::Bo,
            9 => Rotation::Boc,
            10 => Rotation::T,
            11 => Rotation::Tc,
            _ => {
                panic!("Invalid rotation index: {}", i);
            }
        }
    }

    pub fn array() -> [Rotation; 12] {
        [
            Rotation::L,
            Rotation::Lc,
            Rotation::R,
            Rotation::Rc,
            Rotation::Ba,
            Rotation::Bac,
            Rotation::F,
            Rotation::Fc,
            Rotation::Bo,
            Rotation::Boc,
            Rotation::T,
            Rotation::Tc,
        ]
    }

    pub fn is_reverse(&self, other: Rotation) -> bool {
        match (self, other) {
            (Rotation::L, Rotation::Lc) | (Rotation::Lc, Rotation::L) => true,
            (Rotation::R, Rotation::Rc) | (Rotation::Rc, Rotation::R) => true,
            (Rotation::Ba, Rotation::Bac) | (Rotation::Bac, Rotation::Ba) => true,
            (Rotation::F, Rotation::Fc) | (Rotation::Fc, Rotation::F) => true,
            (Rotation::Bo, Rotation::Boc) | (Rotation::Boc, Rotation::Bo) => true,
            (Rotation::T, Rotation::Tc) | (Rotation::Tc, Rotation::T) => true,
            _ => false,
        }
    }
}

pub struct RotationDetails {
    pub face: Face,
    pub direction: Rdir,
}

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub axis: usize,
    pub value: u8,
}
