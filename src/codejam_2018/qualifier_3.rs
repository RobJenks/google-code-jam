#[path = "../util/mod.rs"] mod util;

type Grid = Vec<Vec<bool>>;

pub fn run() {
    let tests = util::stdin_get().parse::<usize>().unwrap();
    (0..tests)
        .for_each(|_| {
            let target = util::stdin_get().parse::<usize>().unwrap();
            execute(target);
        })
}

fn execute(target: usize) {
    let size = determine_best_size(target);
    let mut grid = build_grid(size);

    while let next = best_cell(&grid, &size) {
        println!("{} {}", next.0, next.1);

        let response = util::stdin_get()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if response.iter().all(|x| *x == 0) { return; }  // Success
        if response.iter().any(|x| *x == -1) { panic!("FAILED"); }

        grid[(response[1] - 1) as usize][(response[0] - 1) as usize] = true;
    }
}

// Determines the best (i.e. closest to square) dimensions for a given target size
fn determine_best_size(target: usize) -> (usize, usize) {
    let root = (target as f32).sqrt();
    let b0 = (1..(root.ceil() as usize))
        .rev()
        .skip_while(|x| target % x != 0)
        .take(1)
        .last()
        .unwrap_or_else(|| panic!("No valid bounds"));

    (std::cmp::max(3, b0), std::cmp::max(3, target / b0))
}

fn build_grid(size: (usize, usize)) -> Grid {
    vec![vec![false; size.0]; size.1]
}

// Determine the best target cell in which to trigger an update
fn best_cell(grid: &Grid, size: &(usize, usize)) -> (usize, usize) {
    let (mut best, mut best_count) = ((0, 0), 10);

    for (y, row) in grid.iter().skip(1).take(size.1 - 2).enumerate() {
        for (x, _) in row.iter().skip(1).take(size.0 - 2).enumerate() {
            let count = prepared_count(grid, (x, y));
            if count < best_count {
                if count == 0 { return (x + 2, y + 2); }    // Can't get better than this

                best = (x, y);
                best_count = count;
            }
        }
    }

    (best.0 + 2, best.1 + 2)
}

// Return the number of cells surrounding the given target which have been prepared
fn prepared_count(grid: &Grid, target: (usize, usize)) -> usize {
    grid
        .iter()
        .skip(target.1)
        .take(3)
        .map(|row| row.iter()
            .skip(target.0)
            .take(3)
            .filter(|x| **x == true)
            .count()
        )
        .sum()
}


#[cfg(test)]
mod tests {
    use super::{Grid, determine_best_size, build_grid};
    use crate::codejam_2018::qualifier_3::best_cell;

    #[test]
    fn test_size() {
        assert_eq!(determine_best_size(20), (4, 5));
        assert_eq!(determine_best_size(200), (10, 20));
    }

    #[test]
    fn test_best_cell() {
        let mut grid = build_grid((4, 4));
        assert_eq!(best_cell(&grid, &(4, 4)), (1, 1));

        grid[1][0] = true;
        grid[0][1] = true;
        assert_eq!(best_cell(&grid, &(4, 4)), (2, 2));
    }
}