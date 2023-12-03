crate::AocDay!(3);

const INPUT: &[u8] = include_bytes!("input.txt");

pub fn part_1() -> String {
    let mat = INPUT
        .split(|&b| b == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    let dim_y = mat.len();
    let dim_x = mat[0].len();

    let mut sum = 0;

    for y in 0..dim_y {
        for x in 0..dim_x {
            let ch = mat[y][x];
            if !ch.is_ascii_digit() && ch != b'.' {
                let mut local_mat = [[0; 3]; 3];
                for position in POSITIONS {
                    let (dx, dy) = position;
                    let matrix_x = (dx + 1) as usize;
                    let matrix_y = (dy + 1) as usize;
                    let abs_x = (x as i32 + dx) as usize;
                    let abs_y = (y as i32 + dy) as usize;
                    local_mat[matrix_y][matrix_x] = read_num(&mat, abs_x, abs_y);
                }

                for line in local_mat {
                    let [x, y, z] = line;

                    if x != 0 {
                        sum += x;
                    }

                    if y != z && z != 0 {
                        sum += z;
                    }

                    if x != y && y != 0 {
                        sum += y;
                    }
                }
            }
        }
    }

    format!("{}", sum)
}

pub fn part_2() -> String {
    let mat = INPUT
        .split(|&b| b == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    let dim_y = mat.len();
    let dim_x = mat[0].len();

    let mut sum = 0;

    for y in 0..dim_y {
        for x in 0..dim_x {
            let ch = mat[y][x];
            if !ch.is_ascii_digit() && ch != b'.' {
                let mut local_mat = [[0; 3]; 3];
                for position in POSITIONS {
                    let (dx, dy) = position;
                    let matrix_x = (dx + 1) as usize;
                    let matrix_y = (dy + 1) as usize;
                    let abs_x = (x as i32 + dx) as usize;
                    let abs_y = (y as i32 + dy) as usize;
                    local_mat[matrix_y][matrix_x] = read_num(&mat, abs_x, abs_y);
                }

                let mut n = 0;
                let mut p = 1;
                for line in local_mat {
                    let [x, y, z] = line;

                    if x != 0 {
                        n += 1;
                        p *= x;
                    }

                    if y != z && z != 0 {
                        n += 1;
                        p *= z;
                    }

                    if x != y && y != 0 {
                        n += 1;
                        p *= y;
                    }
                }

                if n == 2 {
                    sum += p;
                }
            }
        }
    }

    format!("{}", sum)
}

fn read_num(mat: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let mut number_start = x;
    let mut number_end = x;

    if !mat[y][x].is_ascii_digit() {
        return 0;
    }

    while number_start > 0 {
        if !mat[y][number_start - 1].is_ascii_digit() {
            break;
        }
        number_start -= 1;
    }

    while number_end < mat[y].len() - 1 {
        if !mat[y][number_end + 1].is_ascii_digit() {
            break;
        }
        number_end += 1;
    }

    let parts = &mat[y][number_start..number_end + 1];
    if parts.is_empty() {
        return 0;
    }
    let s = std::str::from_utf8(parts);
    let p = s.unwrap().parse::<u32>();
    p.unwrap()
}

const POSITIONS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
