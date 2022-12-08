use crate::AocDay;

#[derive(Default)]
pub struct Parts {}

impl AocDay for Parts {
    fn part_1(&self) -> String {
        p1()
    }

    fn part_2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let trees = include_bytes!("input.txt")
        .iter()
        .filter(|x| **x != b'\n')
        .collect::<Vec<_>>();
    let tree_count = trees.len();
    let grid_size = (tree_count as f64).sqrt() as usize;
    let mut sum = 0usize;

    for i in 0..trees.len() {
        let row = i / (grid_size + 1);
        let col = i % (grid_size + 1);

        // Check if there are taller trees in each direction (up, down, left, right)
        if (0..col).any(|val| {
            // Check trees to the left
            val != col && trees[row * grid_size + val] >= trees[row * grid_size + col]
        }) && (col + 1..grid_size).any(|val| {
            // Check trees to the right
            val != col && trees[row * grid_size + val] >= trees[row * grid_size + col]
        }) && (0..row).any(|val| {
            // Check trees above
            val != row && trees[val * grid_size + col] >= trees[row * grid_size + col]
        }) && (row + 1..grid_size).any(|val| {
            // Check trees below
            val != row && trees[val * grid_size + col] >= trees[row * grid_size + col]
        }) {
            sum += 1; // If the current tree is surrounded, increment the counter
        }
    }

    format!("{:?}", tree_count - sum) // Print the number of non-surrounded trees
}

pub fn p2() -> String {
    let trees = include_bytes!("input.txt")
        .iter()
        .filter(|x| **x != b'\n')
        .collect::<Vec<_>>();
    let tree_count = trees.len();
    let grid_size = (tree_count as f64).sqrt() as usize;
    let mut scenic = 0usize;

    for i in 0..trees.len() {
        let row = i / grid_size;
        let col = i % grid_size;
        let mut cur_scenic = 1usize;

        if let Some(val) = trees[row * grid_size..row * grid_size + col]
            .iter()
            .rev()
            .enumerate()
            .find(|(_, val)| val >= &&trees[row * grid_size + col])
        {
            cur_scenic *= val.0 + 1
        } else {
            cur_scenic *= col
        }

        if let Some(val) = trees[row * grid_size + col + 1..row * grid_size + grid_size]
            .iter()
            .enumerate()
            .find(|(_, val)| val >= &&trees[row * grid_size + col])
        {
            cur_scenic *= val.0 + 1
        } else {
            cur_scenic *= grid_size - (col + 1)
        }

        if let Some(val) = (0..row)
            .rev()
            .enumerate()
            .find(|(_, val)| trees[val * grid_size + col] >= trees[row * grid_size + col])
        {
            cur_scenic *= val.0 + 1
        } else {
            cur_scenic *= row
        }

        if let Some(val) = (row + 1..grid_size)
            .enumerate()
            .find(|(_, val)| trees[val * grid_size + col] >= trees[row * grid_size + col])
        {
            cur_scenic *= val.0 + 1
        } else {
            cur_scenic *= grid_size - (row + 1)
        }

        scenic = std::cmp::max(scenic, cur_scenic);
    }

    format!("{:?}", scenic)
}
