crate::AocDay!(2);

const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> String {
    const LIMIT_R: u16 = 12;
    const LIMIT_G: u16 = 13;
    const LIMIT_B: u16 = 14;

    let mut sum = 0;
    'l: for (game_id, line) in INPUT.lines().enumerate() {
        let game_id = game_id + 1;
        let data_delim = line.find(':').unwrap();
        let line = &line[data_delim + 2..]; // skip ": "

        for drawing in line.split("; ") {
            for cube_draw in drawing.split(", ") {
                let (number, color) = cube_draw.split_once(' ').unwrap();
                let number = number.parse::<u16>().unwrap();

                match color.as_bytes()[0] {
                    b'r' => {
                        if number > LIMIT_R {
                            continue 'l;
                        }
                    }
                    b'b' => {
                        if number > LIMIT_B {
                            continue 'l;
                        }
                    }
                    b'g' => {
                        if number > LIMIT_G {
                            continue 'l;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        sum += game_id;
    }

    format!("{}", sum)
}

pub fn part_2() -> String {
    let mut sum = 0;
    for line in INPUT.lines() {
        let data_delim = line.find(':').unwrap();
        let line = &line[data_delim + 2..]; // skip ": "

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for drawing in line.split("; ") {
            for cube_draw in drawing.split(", ") {
                let (number, color) = cube_draw.split_once(' ').unwrap();
                let number = number.parse::<u16>().unwrap();

                match color.as_bytes()[0] {
                    b'r' => r = r.max(number),
                    b'g' => g = g.max(number),
                    b'b' => b = b.max(number),
                    _ => unreachable!(),
                }
            }
        }

        sum += r * g * b;
    }

    format!("{}", sum)
}
