use std::ops::Range;

use itertools::Itertools;

crate::AocDay!(5);

const INPUT: &str = include_str!("input.txt");

#[inline(always)]
pub fn part_1() -> String {
    let s = INPUT.split("\n\n").collect_vec();
    let seeds = parse_seeds(s[0]);
    let categories = parse_categories(s);

    let mut outputs = Vec::new();
    for seed in seeds {
        let mut output = seed;
        for category in &categories {
            output = category.map(output);
        }
        outputs.push(output);
    }

    let min = outputs.iter().min().unwrap();

    format!("{}", min)
}

#[inline(always)]
pub fn part_2() -> String {
    format!("")
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let mut seeds = Vec::new();
    let seed_s = input.split(" ").skip(1);
    for seed in seed_s {
        seeds.push(seed.parse::<usize>().unwrap());
    }
    seeds
}

fn parse_categories(s: Vec<&str>) -> Vec<Category> {
    let mut categories = Vec::new();
    for category in &s[1..] {
        let cat = parse_category(category);
        categories.push(cat);
    }
    categories
}

fn parse_category(input: &str) -> Category {
    let mut maps = Vec::new();
    let lines = input.split("\n").skip(1).collect_vec();
    for line in lines {
        let mut s = line.split(" ");
        let target = s.next().unwrap().parse::<usize>().unwrap();
        let source = s.next().unwrap().parse::<usize>().unwrap();
        let len = s.next().unwrap().parse::<usize>().unwrap();
        let map = Map::new(source..source + len, target..target + len);
        maps.push(map);
    }
    Category::new(maps)
}

#[derive(Debug, Clone)]
struct Map {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Map {
    fn new(source: Range<usize>, destination: Range<usize>) -> Self {
        Self {
            source,
            destination,
        }
    }

    fn map(&self, input: usize) -> Option<usize> {
        if self.source.contains(&input) {
            let offset = input - self.source.start;
            Some(self.destination.start + offset)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Category {
    maps: Vec<Map>,
}

impl Category {
    fn new(maps: Vec<Map>) -> Self {
        Self { maps }
    }

    pub fn map(&self, input: usize) -> usize {
        for map in &self.maps {
            if let Some(output) = map.map(input) {
                return output;
            }
        }
        input
    }
}
