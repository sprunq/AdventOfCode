use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

crate::AocDay!(5);

const INPUT: &str = include_str!("input.txt");

#[inline(always)]
pub fn part_1() -> String {
    let s = INPUT.split("\n\n").collect_vec();
    let seeds = parse_seeds(s[0]);
    let categories = parse_categories(s);

    let min = seeds
        .iter()
        .map(|seed| {
            let mut output = *seed;
            for category in &categories {
                output = category.map(output);
            }
            output
        })
        .min()
        .unwrap();

    format!("{}", min)
}

#[inline(always)]
pub fn part_2() -> String {
    let s = INPUT.split("\n\n").collect_vec();
    let seeds = parse_seed_ranges(s[0]);
    let categories = parse_categories(s);

    let res_len = seeds.iter().map(|x| x.len()).sum::<usize>();
    dbg!(res_len);

    let min = seeds
        .into_iter()
        .map(|sr| {
            println!("{:?}", sr);

            sr.into_par_iter()
                .map(|s| {
                    let mut output = s;
                    for category in &categories {
                        output = category.map(output);
                    }
                    output
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    format!("{}", min)
}

#[derive(Debug, Clone)]
struct Map {
    src: usize,
    dst: usize,
    len: usize,
}

impl Map {
    fn new(src: usize, dst: usize, len: usize) -> Self {
        Self { src, dst, len }
    }

    #[inline(always)]
    fn map(&self, input: usize) -> Option<usize> {
        if input >= self.src && input < self.src + self.len {
            let offset = input - self.src;
            Some(self.dst + offset)
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

    #[inline(always)]
    pub fn map(&self, input: usize) -> usize {
        for map in &self.maps {
            if let Some(output) = map.map(input) {
                return output;
            }
        }
        input
    }
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let mut seeds = Vec::new();
    let seed_s = input.split(' ').skip(1);
    for seed in seed_s {
        seeds.push(seed.parse::<usize>().unwrap());
    }
    seeds
}

fn parse_seed_ranges(input: &str) -> Vec<Range<usize>> {
    let seed_s = input.split(' ').skip(1).collect_vec();
    seed_s
        .chunks_exact(2)
        .map(|x| {
            let start = x[0].parse::<usize>().unwrap();
            let len = x[1].parse::<usize>().unwrap();
            start..start + len
        })
        .collect_vec()
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
    let lines = input.split('\n').skip(1).collect_vec();
    for line in lines {
        let mut s = line.split(' ');
        let target = s.next().unwrap().parse::<usize>().unwrap();
        let source = s.next().unwrap().parse::<usize>().unwrap();
        let len = s.next().unwrap().parse::<usize>().unwrap();
        let map = Map::new(source, target, len);
        maps.push(map);
    }
    Category::new(maps)
}
