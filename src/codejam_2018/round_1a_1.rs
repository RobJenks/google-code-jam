#[path = "../util/mod.rs"]
mod util;
use std;

type Grid = Vec<Vec<bool>>;

pub fn run() {
    let mut input = util::Input::create(true, "./src/codejam_2018/input-r1a-1");
    let tests = input.get_line_as::<usize>();
    (0..tests)
        .for_each(|t| {
            let params = input.get_line_components_as::<usize>();   // [ Rows, Cols, H-Splits, V-Splits]
            let (r, c, h, v) = (params[0], params[1], params[2], params[3]);

            let data = load_grid(&mut input, r);

            println!("Case #{}: {}", t,
                 match solve(&data, r, c, h, v) {
                     true => "POSSIBLE",
                     false => "IMPOSSIBLE"
                 }
            );
        });
}

fn load_grid(input: &mut util::Input, rows: usize) -> Grid {
    (0..rows)
        .map(|_| input.get_line())
        .map(|l| l.chars()
            .map(|c| c == '@')
            .collect::<Vec<bool>>()
        )
        .collect::<Grid>()
}

fn solve(grid: &Grid, rows: usize, cols: usize, hsplits: usize, vsplits: usize) -> bool {
    // Need to be able to split into equal one-dimensional slices, in both directions
    let total = grid.iter().map(|row| row.iter().filter(|&&x| x).count() as u32).sum::<u32>();

    let (row_cc, col_cc) = cumulative_counts(grid, rows, cols);
    let (h_eq, v_eq) = (total / (hsplits as u32 + 1), total / (vsplits as u32 + 1));

    // Will require splits at each of these intervals
    let req_h = (0..hsplits as u32).map(|x| (x+1) * h_eq).collect::<Vec<u32>>();
    let req_v = (0..vsplits as u32).map(|x| (x+1) * v_eq).collect::<Vec<u32>>();

    let hsplits = get_cumulative_indices(&row_cc, &req_h);
    let vsplits = get_cumulative_indices(&col_cc, &req_v);

    hsplits.is_some() && vsplits.is_some()
        && equally_distributed(&get_distribution(&grid, &hsplits.unwrap(), &vsplits.unwrap()))
}

fn cumulative_counts(grid: &Grid, rows: usize, cols: usize) -> (Vec<u32>, Vec<u32>) {
    // Get the number of items in each row and col separately
    let row_dist = get_distribution(&grid, &(0..rows).collect::<Vec<usize>>(), &vec![]);
    let col_dist = get_distribution(&grid, &vec![], &(0..cols).collect::<Vec<usize>>());

    let row_count = row_dist.iter().map(|x| *x.get(0).unwrap()).collect::<Vec<u32>>();
    let col_count = col_dist.get(0).unwrap().clone();

    let c_sum = |v: &Vec<u32>| v
        .iter()
        .scan(0u32, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<u32>>();

    (c_sum(&row_count), c_sum(&col_count))
}

fn get_distribution(grid: &Grid, hsplits: &Vec<usize>, vsplits: &Vec<usize>) -> Vec<Vec<u32>> {
    if grid.is_empty() { return vec![]; }

    let mut count = vec![vec![0u32; vsplits.len() + 1]; hsplits.len() + 1];
    let (mut hsi, mut vsi) = (0usize, 0usize);      // Split that we are currently in

    grid.iter()
        .enumerate()
        .for_each(|(y, row)| {
            if hsi < hsplits.len() && y == hsplits[hsi] {      // See if we moved into a new hsplit
                hsi += 1;
            }

            vsi = 0;
            row.iter()
                .enumerate()
                .for_each(|(x, cell)| {
                    if vsi < vsplits.len() && x == vsplits[vsi] {    // See if we moved into a new vsplit
                        vsi += 1;
                    }

                    if *cell {       // Record new item in the relevant split
                        count[hsi][vsi] += 1;
                    }
                })
        });

        count
}

fn equally_distributed(dist: &Vec<Vec<u32>>) -> bool {
    let val = dist[0][0];
    dist.iter().all(|row| row.iter().all(|x| *x == val))
}

fn get_cumulative_indices(values: &Vec<u32>, targets: &Vec<u32>) -> Option<Vec<usize>> {
    let mut indices = vec![];
    let mut tgt = 0usize;

    values.iter().enumerate().for_each(|(i, &x)| {
        if tgt >= targets.len() { return; }
        else if x == targets[tgt] {
            indices.push(i);
            tgt += 1;
        }

        else if x > targets[tgt] { return; }    // Failed
    });

    if indices.len() == targets.len() { Some(indices) } else { None }
}


#[cfg(test)]
mod tests {
    //#[path = "../../../util/mod.rs"] mod util;
    use super::{util::Input, Grid, load_grid, get_distribution };

    #[test]
    fn test_dist() {
        verify_dist("@@@\n@@@\n@@@", &vec![1, 2], &vec![1, 2],
                    &vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]);

        verify_dist("@..\n.@.\n..@", &vec![1, 2], &vec![1, 2],
                    &vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);

        verify_dist("..\n..\n..", &vec![1], &vec![1],
                    &vec![vec![0, 0], vec![0, 0]]);

        verify_dist("@@@\n@@@\n@@@", &vec![2], &vec![],
                    &vec![vec![6], vec![3]]);

        verify_dist("@@@\n@@@\n@@@", &vec![], &vec![1],
                    &vec![vec![3, 6]]);

        verify_dist("@@@\n@@@\n@@@", &vec![], &vec![],
                    &vec![vec![9]]);

        verify_dist("@", &vec![], &vec![],
                    &vec![vec![1]]);

        verify_dist(".", &vec![], &vec![],
                    &vec![vec![0]]);

    }

    fn verify_dist(grid_data: &str, hsplits: &Vec<usize>, vsplits: &Vec<usize>, expected_dist: &Vec<Vec<u32>>) {
        let mut input = Input::create_for_string(grid_data);
        let grid = load_grid(&mut input, grid_data.split("\n").count());

        let dist = get_distribution(&grid, &hsplits, &vsplits);
        assert_eq!(dist, *expected_dist);
    }
}