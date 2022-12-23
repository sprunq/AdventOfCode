use itertools::Itertools;

use crate::Solution;
use std::{cmp::Ordering, fmt, iter::Peekable, str::Chars};

#[derive(Default)]
pub struct Parts {}

impl Solution for Parts {
    fn part_1(&self) -> String {
        p1()
    }

    fn part_2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let sum = include_str!("input.txt")
        .split("\n\n")
        .map(|e| e.split('\n').collect::<Vec<_>>())
        .map(|pair| {
            pair.iter()
                .map(|line| parse_node(&mut line.chars().peekable()))
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .enumerate()
        .map(|(idx, (a, b))| {
            if let Ordering::Less = a.cmp(&b) {
                idx + 1
            } else {
                0
            }
        })
        .sum::<usize>();
    format!("{:?}", sum)
}

pub fn p2() -> String {
    let divs = [
        Node::Nodes(vec![Node::Nodes(vec![Node::Value(2)])]),
        Node::Nodes(vec![Node::Nodes(vec![Node::Value(6)])]),
    ];
    let res = include_str!("input.txt")
        .split("\n\n")
        .map(|e| e.split('\n').collect::<Vec<_>>())
        .map(|pair| {
            pair.iter()
                .map(|line| parse_node(&mut line.chars().peekable()))
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .flat_map(|(a, b)| [a, b])
        .chain(divs.clone())
        .sorted()
        .positions(|n| n == divs[0] || n == divs[1])
        .map(|pos| pos + 1)
        .product::<usize>();

    format!("{:?}", res)
}

fn parse_node(chars: &mut Peekable<Chars>) -> Node {
    let mut items = Vec::new();
    chars.next();
    while let Some(peek_ch) = chars.peek() {
        match peek_ch {
            '[' => {
                let node = parse_node(chars);
                items.push(node);
            }
            '0'..='9' => {
                let num_str = chars
                    .peeking_take_while(|c| matches!(c, '0'..='9'))
                    .collect::<String>();
                let number = num_str.parse().unwrap();
                items.push(Node::Value(number));
            }
            ']' => {
                chars.next();
                break;
            }
            ',' => {
                chars.next();
            }
            _ => unreachable!(),
        }
    }
    Node::Nodes(items)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Value(u8),
    Nodes(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Value(a), Node::Value(b)) => a.cmp(b),
            (Node::Nodes(a), Node::Nodes(b)) => {
                for index in 0..a.len().min(b.len()) {
                    let order = a[index].cmp(&b[index]);
                    match order {
                        Ordering::Equal => (),
                        _ => return order,
                    }
                }

                a.len().cmp(&b.len())
            }
            (Node::Value(a), Node::Nodes(_)) => Node::Nodes(vec![Node::Value(*a)]).cmp(other),
            (Node::Nodes(_), Node::Value(b)) => self.cmp(&Node::Nodes(vec![Node::Value(*b)])),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Node::Value(val) => write!(f, "{}", val),
            Node::Nodes(nodes) => {
                write!(
                    f,
                    "[{}]",
                    nodes
                        .iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}
