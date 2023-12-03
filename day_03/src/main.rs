#![allow(unused)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "4361";
    let ga1 = ga1.to_string();

    let i2 = include_str!("i2.txt").trim();
    let e2 = include_str!("e2.txt").trim();
    let ga2 = "467835";
    let ga2 = ga2.to_string();

    if e1.is_empty() {
        panic!("e1.txt empty dumbass");
    }
    let ea1 = part1(e1);
    assert!(!ga1.is_empty(), "ga1 empty dumbass");
    assert_eq!(ga1, ea1);
    if i1.is_empty() {
        panic!("i1.txt empty dumbass");
    }
    let a1 = part1(i1);
    println!("--- PART1: {a1}");
    let ca1 = 540212;
    let ca1 = ca1.to_string();
    if ca1.is_empty() {
        panic!("save the answer in ca1 before you f up");
    }
    assert_eq!(ca1, a1);

    if e2.is_empty() {
        panic!("e2.txt empty dumbass");
    }
    let ea2 = part2(e2);
    assert_ne!(ga2, "", "ga2 empty dumbass");
    assert_eq!(ga2, ea2);
    if i2.is_empty() {
        panic!("i2.txt empty dumbass");
    }
    let a2 = part2(i2);
    println!("--- PART2: {a2}");
    let ca2 = "87605697";
    let ca2 = ca2.to_string();
    if ca2.is_empty() {
        panic!("save the answer in ca2 before you f up");
    }
    assert_eq!(ca2, a2);
}

use std::{
    collections::HashSet,
    ops::{Index, Range},
};

#[derive(Clone, Debug)]
enum InputTypes {
    Nothing,
    Part(usize, char),
    Number(Range<usize>, u32),
}

pub fn idx_to_cord(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

fn cord_to_idx(x: usize, y: usize, width: usize) -> usize {
    (y) * width + (x)
}

fn parse(input: &str) -> Vec<InputTypes> {
    let mut vec_char = vec![];
    let mut output = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, it) in line.trim().chars().enumerate() {
            let idx = cord_to_idx(x, y, line.trim().len());
            if it.is_whitespace() {
                continue;
            }
            if it.is_ascii_digit() {
                vec_char.push((idx, it));
                continue;
            }
            let op = match it {
                '.' => InputTypes::Nothing,
                c => InputTypes::Part(idx, c),
            };
            if !vec_char.is_empty() {
                output.push({
                    InputTypes::Number(
                        vec_char
                            .iter()
                            .min_by(|(idx1, _), (idx2, _)| idx1.cmp(idx2))
                            .unwrap()
                            .0
                            ..vec_char
                                .iter()
                                .max_by(|(idx1, _), (idx2, _)| idx1.cmp(idx2))
                                .unwrap()
                                .0
                                + 1,
                        vec_char
                            .iter()
                            .map(|(_, it)| it)
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    )
                });
                vec_char.clear()
            }
            output.push(op);
        }
    }
    output
}

fn part1(input: &str) -> String {
    let parsed_input = parse(input);
    let width = (input.lines().next().unwrap().len());
    let parts = parsed_input
        .iter()
        .filter_map(|it| {
            if let InputTypes::Part(idx, c) = it {
                Some((*idx, *c))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let part_numbers = parsed_input
        .iter()
        .filter_map(|it| {
            if let InputTypes::Number(range, number) = it {
                Some((range.clone(), *number))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    parts
        .into_iter()
        .flat_map(|(idx, _c)| {
            let current_cords = idx_to_cord(idx, width);
            let cords = (-1..2)
                .flat_map(|y_off| {
                    (-1..2)
                        .filter_map(|x_off| {
                            if let (Some(x), Some(y)) = (
                                (current_cords.0 as i64).checked_add(x_off),
                                (current_cords.1 as i64).checked_add(y_off),
                            ) {
                                if x < 0 || y < 0 {
                                    None
                                } else {
                                    Some((x as usize, y as usize))
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let idxs = cords
                .clone()
                .iter()
                .map(|(x, y)| cord_to_idx(*x, *y, width))
                .collect::<Vec<_>>();

            let mut nums = HashSet::new();

            for idx in idxs {
                let filtered_list = part_numbers
                    .iter()
                    .filter(|(range, _)| range.contains(&idx));
                for it in filtered_list {
                    nums.insert(it.clone());
                }
            }
            nums
        })
        .collect::<HashSet<(_, _)>>()
        .iter()
        .map(|it| (it).1)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let parsed_input = parse(input);
    let width = (input.lines().next().unwrap().len());
    let parts = parsed_input
        .iter()
        .filter_map(|it| {
            if let InputTypes::Part(idx, '*') = it {
                Some((*idx, '*'))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let part_numbers = parsed_input
        .iter()
        .filter_map(|it| {
            if let InputTypes::Number(range, number) = it {
                Some((range.clone(), *number))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    parts
        .into_iter()
        .map(|(idx, _c)| {
            let current_cords = idx_to_cord(idx, width);
            let cords = (-1..2)
                .flat_map(|y_off| {
                    (-1..2)
                        .filter_map(|x_off| {
                            if let (Some(x), Some(y)) = (
                                (current_cords.0 as i64).checked_add(x_off),
                                (current_cords.1 as i64).checked_add(y_off),
                            ) {
                                if x < 0 || y < 0 {
                                    None
                                } else {
                                    Some((x as usize, y as usize))
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let idxs = cords
                .clone()
                .iter()
                .map(|(x, y)| cord_to_idx(*x, *y, width))
                .collect::<Vec<_>>();

            let mut nums = HashSet::new();

            for idx in idxs {
                let filtered_list = part_numbers
                    .iter()
                    .filter(|(range, _)| range.contains(&idx));
                for it in filtered_list {
                    nums.insert(it.clone());
                }
            }
            (idx, nums)
        })
        .filter(|(_, it)| it.len() == 2)
        .map(|(i, set)| {
            let vec = set.into_iter().collect::<Vec<_>>();
            vec.into_iter().map(|(_, it)| it).product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}
