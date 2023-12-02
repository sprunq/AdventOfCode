crate::AocDay!(2);

const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> String {
    let parsed = parse_input(INPUT);
    const LIMIT_RED: u16 = 12;
    const LIMIT_GREEN: u16 = 13;
    const LIMIT_BLUE: u16 = 14;

    let possibe_games: usize = parsed
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            let possible_games = game
                .iter()
                .filter(|draw| draw.0 <= LIMIT_RED && draw.1 <= LIMIT_GREEN && draw.2 <= LIMIT_BLUE)
                .count();

            if possible_games == game.len() {
                Some(id + 1)
            } else {
                None
            }
        })
        .sum();

    format!("{}", possibe_games)
}

pub fn part_2() -> String {
    let parsed = parse_input(INPUT);

    let possibe_games: u16 = parsed
        .iter()
        .map(|game| {
            let max_red = game.iter().map(|draw| draw.0).max().unwrap();
            let max_green = game.iter().map(|draw| draw.1).max().unwrap();
            let max_blue = game.iter().map(|draw| draw.2).max().unwrap();

            max_red * max_green * max_blue
        })
        .sum();

    format!("{}", possibe_games)
}

fn parse_input(input: &str) -> Vec<Vec<(u16, u16, u16)>> {
    let lines = input.lines();
    let mut games = Vec::new();

    for line in lines {
        let (_, game) = line.split_once(":").unwrap();
        let drawings = game.split(";");
        let mut game_draws = Vec::new();
        for drawing in drawings {
            game_draws.push(parse_drawing(drawing));
        }
        games.push(game_draws);
    }
    games
}

fn parse_drawing(input: &str) -> (u16, u16, u16) {
    let split = input.split(",");
    let mut draw = (0, 0, 0);
    for s in split {
        let (digit, color) = s.trim().split_once(" ").unwrap();
        let number = digit.trim().parse::<u16>().unwrap();

        match color.trim() {
            "red" => {
                draw.0 = number;
            }
            "blue" => {
                draw.1 = number;
            }
            "green" => {
                draw.2 = number;
            }
            _ => panic!("Invalid color"),
        }
    }
    draw
}
