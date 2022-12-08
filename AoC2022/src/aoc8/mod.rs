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
    let total_trees = trees.len();
    let grid_s = (total_trees as f64).sqrt() as usize;
    let mut surrounded_trees = 0usize;

    for i in 0..trees.len() {
        let r = i / (grid_s + 1);
        let col = i % (grid_s + 1);

        // Check if there are taller trees in each direction (up, down, left, right)
        if (0..col).any(|v| {
            // Check trees to the left
            v != col && trees[r * grid_s + v] >= trees[r * grid_s + col]
        }) && (col + 1..grid_s).any(|val| {
            // Check trees to the right
            val != col && trees[r * grid_s + val] >= trees[r * grid_s + col]
        }) && (0..r).any(|v| {
            // Check trees above
            v != r && trees[v * grid_s + col] >= trees[r * grid_s + col]
        }) && (r + 1..grid_s).any(|v| {
            // Check trees below
            v != r && trees[v * grid_s + col] >= trees[r * grid_s + col]
        }) {
            surrounded_trees += 1; // If the current tree is surrounded, increment the counter
        }
    }

    format!("{:?}", total_trees - surrounded_trees) // Print the number of non-surrounded trees
}

pub fn p2() -> String {
    let trees = include_bytes!("input.txt")
        .iter()
        .filter(|x| **x != b'\n')
        .collect::<Vec<_>>();
    let tree_count = trees.len();
    let grid_size = (tree_count as f64).sqrt() as usize;
    let mut scenic_score = 0usize;

    for i in 0..trees.len() {
        let row = i / grid_size;
        let col = i % grid_size;
        let mut cur_scenic_score = 1usize;

        if let Some(val) = trees[row * grid_size..row * grid_size + col]
            .iter()
            .rev()
            .enumerate()
            .find(|(_, val)| val >= &&trees[row * grid_size + col])
        {
            cur_scenic_score *= val.0 + 1
        } else {
            cur_scenic_score *= col
        }

        if let Some(val) = trees[row * grid_size + col + 1..row * grid_size + grid_size]
            .iter()
            .enumerate()
            .find(|(_, val)| val >= &&trees[row * grid_size + col])
        {
            cur_scenic_score *= val.0 + 1
        } else {
            cur_scenic_score *= grid_size - (col + 1)
        }

        if let Some(val) = (0..row)
            .rev()
            .enumerate()
            .find(|(_, val)| trees[val * grid_size + col] >= trees[row * grid_size + col])
        {
            cur_scenic_score *= val.0 + 1
        } else {
            cur_scenic_score *= row
        }

        if let Some(val) = (row + 1..grid_size)
            .enumerate()
            .find(|(_, val)| trees[val * grid_size + col] >= trees[row * grid_size + col])
        {
            cur_scenic_score *= val.0 + 1
        } else {
            cur_scenic_score *= grid_size - (row + 1)
        }

        scenic_score = std::cmp::max(scenic_score, cur_scenic_score);
    }

    format!("{:?}", scenic_score)
}
