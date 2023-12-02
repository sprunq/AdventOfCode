crate::AocDay!(2);

const INPUT: &str = include_str!("input.txt");

pub fn part_1() -> String {
    let parsed = parse_input(INPUT);
    const LIMIT_RED: usize = 12;
    const LIMIT_GREEN: usize = 13;
    const LIMIT_BLUE: usize = 14;

    let possibe_games: usize = parsed
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            let possible_games = game
                .iter()
                .filter(|draw| {
                    draw.red <= LIMIT_RED && draw.green <= LIMIT_GREEN && draw.blue <= LIMIT_BLUE
                })
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

    let possibe_games: usize = parsed
        .iter()
        .map(|game| {
            let max_red = game.iter().map(|draw| draw.red).max().unwrap();
            let max_green = game.iter().map(|draw| draw.green).max().unwrap();
            let max_blue = game.iter().map(|draw| draw.blue).max().unwrap();

            max_red * max_green * max_blue
        })
        .sum();

    format!("{}", possibe_games)
}

fn parse_input(input: &str) -> Vec<Vec<GameDraw>> {
    let mut games = Vec::new();
    for line in input.lines() {
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

fn parse_drawing(input: &str) -> GameDraw {
    let split = input.split(",");
    let mut draw = GameDraw::empty();
    for s in split {
        let (digit, color) = s.trim().split_once(" ").unwrap();
        let number = digit.trim().parse::<usize>().unwrap();

        match color.trim() {
            "red" => {
                draw.red = number;
            }
            "blue" => {
                draw.blue = number;
            }
            "green" => {
                draw.green = number;
            }
            _ => panic!("Invalid color"),
        }
    }
    draw
}

#[derive(Debug, Clone)]
struct GameDraw {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameDraw {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
