#[derive(Debug, Copy, Clone)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green
}

#[derive(Debug)]
struct Corner {
    // Position tuples are (x, y, z) coordinates of the pieces. So the corner at
    // (0, 0, 0) is touching the origin point, and (0, 0, 1) is the piece right
    // above that.
    position: (u8, u8, u8),

    // Orientation tuples have the visible face colors of the corner in order
    // by the side on the x/y plane, the x/z plane, and the y/z plane.
    orientation: (Color, Color, Color)
}

#[derive(Debug)]
struct PocketCube([Corner; 8]);

impl PocketCube {
    fn getSolvedCube() -> PocketCube {
        let positions = [
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1)
        ];
        let orientations = [
            (Color::White, Color::Green, Color::Orange),
            (Color::Yellow, Color::Green, Color::Orange),
            (Color::White, Color::Blue, Color::Orange),
            (Color::Yellow, Color::Blue, Color::Orange),
            (Color::White, Color::Green, Color::Red),
            (Color::Yellow, Color::Green, Color::Red),
            (Color::White, Color::Blue, Color::Red),
            (Color::Yellow, Color::Blue, Color::Red)
        ];
        let corners: [Corner; 8];
        for i in 0..7 {
            corners[i] = Corner {
                position: positions[i],
                orientation: orientations[i]
            };
        }
        return PocketCube(corners);
    }
}

fn main() {
    let solvedCube = PocketCube::getSolvedCube();
    println!("{:?}", solvedCube);
}
