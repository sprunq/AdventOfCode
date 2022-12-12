use std::collections::VecDeque;

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
    let mut tile_cost = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    let mut start_pos = (0, 0);
    'outer: for r in 0..tile_cost.len() {
        for c in 0..tile_cost[r].len() {
            if tile_cost[r][c] == b'E' {
                start_pos = (r, c);
                break 'outer;
            }
        }
    }
    format!("{:?}", bfs(&mut tile_cost, start_pos, b'S'))
}

pub fn p2() -> String {
    let mut tile_cost = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    let mut start_pos = (0, 0);
    'outer: for r in 0..tile_cost.len() {
        for c in 0..tile_cost[r].len() {
            if tile_cost[r][c] == b'E' {
                start_pos = (r, c);
                break 'outer;
            }
        }
    }
    format!("{:?}", bfs(&mut tile_cost, start_pos, b'a'))
}

fn bfs(tile_weights: &mut Vec<Vec<u8>>, pos: (usize, usize), target: u8) -> Option<usize> {
    const DIR: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    tile_weights[pos.0][pos.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((pos, b'z'));

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            if let Some((p, w)) = queue.pop_front() {
                if target == w {
                    return Some(steps);
                };

                for (dr, dc) in DIR.iter() {
                    let horizontal_x = p.0 as isize + dr;
                    let vertical_x = p.1 as isize + dc;
                    if horizontal_x < 0 || vertical_x < 0 {
                        continue;
                    }
                    let (rx, cx) = (horizontal_x as usize, vertical_x as usize);
                    if rx >= tile_weights.len() || cx >= tile_weights[rx].len() {
                        continue;
                    }
                    let mut new_h = tile_weights[rx][cx];
                    new_h = if new_h == b'S' { b'a' } else { new_h };

                    if new_h == 0 || (new_h < w && (w - new_h) > 1) {
                        continue;
                    }

                    queue.push_back(((rx, cx), tile_weights[rx][cx]));
                    tile_weights[rx][cx] = 0;
                }
            }
        }
        steps += 1;
    }
    None
}
