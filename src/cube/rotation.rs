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

impl Iterator for Rotation {
    type Item = Rotation;
    fn next(&mut self) -> Option<Rotation> {
        match self {
            L => Some(Rotation::Lc),
            Lc => Some(Rotation::R),
            R => Some(Rotation::Rc),
            Rc => Some(Rotation::Ba),
            Ba => Some(Rotation::Bac),
            Bac => Some(Rotation::F),
            F => Some(Rotation::Fc),
            Fc => Some(Rotation::Bo),
            Bo => Some(Rotation::Boc),
            Boc => Some(Rotation::T),
            T => Some(Rotation::Tc),
            Tc => None,
        }
    }
}

pub struct RotationDetails {
    pub face: Face,
    pub direction: Rdir,
}

fn get_rotation(r: Rotation) -> RotationDetails {
    match r {
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

// To select the bottom face, axis would be Z (position tuple index of 2) and
// value would be 0
#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub axis: usize,
    pub value: u8,
}
