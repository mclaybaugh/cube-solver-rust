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

impl Color {
    pub fn to_num(&self) -> u8 {
        match self {
            Color::White => 0,
            Color::Yellow => 1,
            Color::Red => 2,
            Color::Orange => 3,
            Color::Blue => 4,
            Color::Green => 5,
        }
    }

    pub fn from_num(x: u8) -> Color {
        match x {
            0 => Color::White,
            1 => Color::Yellow,
            2 => Color::Red,
            3 => Color::Orange,
            4 => Color::Blue,
            5 => Color::Green,
            _ => {
                panic!("Invalid color num: {}", x)
            }
        }
    }

    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Yellow,
            Color::Yellow => Color::White,
            Color::Red => Color::Orange,
            Color::Orange => Color::Red,
            Color::Blue => Color::Green,
            Color::Green => Color::Blue,
        }
    }
}
