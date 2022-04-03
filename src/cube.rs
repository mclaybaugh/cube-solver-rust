pub trait Cube {
    fn rotate(&self, rotation_index: u8) -> Self;
    fn is_solved(&self) -> bool;
}

// Perform n random rotations on cube and return.
// TODO make this work on any type with trait of "puzzle_cube"
pub fn scramble<T: Cube + Clone>(cube: &T, n: u8) -> (T, Vec<u8>) {
    let mut rotations: Vec<u8> = Vec::new();
    let mut new_cube = cube.clone();
    for _ in 0..n {
        let rotation_index = random_rotation_index();
        rotations.push(rotation_index);
        new_cube = new_cube.rotate(rotation_index);
    }
    return (new_cube, rotations);
}

// Brute force every move and return true if one solves it.
// iterative deepening depth-first traversal
// needs to return breadcrumb tail on success, or false/None
pub fn maybe_solve_in<T: Cube, Clone>(cube: &T, n: u8) -> (Option<Vec<u8>>, u64) {
    let mut depth = 0;
    let mut nodes_checked = 0;
    while depth <= n {
        let moves: Vec<u8> = Vec::new();
        let (returned_moves, is_solved, nodes_checked_r) =
            PocketCube::depth_limited_search(cube.clone(), &moves, depth, nodes_checked);
        nodes_checked = nodes_checked_r;
        if is_solved {
            return (Some(returned_moves.clone()), nodes_checked);
        }
        depth = depth + 1;
    }
    return (None, nodes_checked);
}

fn depth_limited_search<T: Cube, Clone>(
    cube: T,
    moves: &Vec<u8>,
    n: u8,
    mut nodes_checked: u64,
) -> (Vec<u8>, bool, u64) {
    // end of line, check if solved
    if n == 0 {
        if cube.is_solved() {
            return (moves.clone(), true, nodes_checked + 1);
        } else {
            return (moves.clone(), false, nodes_checked + 1);
        }
    }

    // do check on children (rotations)
    for i in 0..12 {
        let move_len = moves.len();
        // don't reverse prior move
        if move_len > 0 && is_reverse_rotation(moves[move_len - 1], i) {
            continue;
        }
        // don't do same move 4 times in a row
        if move_len > 3
            && (moves[move_len - 1] == i && moves[move_len - 2] == i && moves[move_len - 3] == i)
        {
            continue;
        }

        let mut new_moves = moves.clone();
        new_moves.push(i);
        let ncube = cube.rotate(i);
        let (new_moves, solved, nodes_checked_r) =
            depth_limited_search(ncube, &new_moves, n - 1, nodes_checked);
        nodes_checked = nodes_checked_r;
        if solved {
            return (new_moves.clone(), solved, nodes_checked);
        }
    }
    return (moves.clone(), false, nodes_checked);
}
