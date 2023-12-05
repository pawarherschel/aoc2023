#![allow(unused)]

use indicatif::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    char::REPLACEMENT_CHARACTER,
    collections::{
        btree_map::{Keys, Values},
        HashMap,
    },
    time::Instant,
};

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "35";
    let ga1 = ga1.to_string();
    let ca1 = "";
    let ca1 = ca1.to_string();

    let i2 = include_str!("i2.txt").trim();
    let e2 = include_str!("e2.txt").trim();
    let ga2 = "";
    let ga2 = ga2.to_string();
    let ca2 = "";
    let ca2 = ca2.to_string();

    if e1.is_empty() {
        panic!("e1.txt empty dumbass");
    }
    assert!(!ga1.is_empty(), "ga1 empty dumbass");
    let now = Instant::now();
    let ea1 = format!("{:?}", part1(e1));
    println!("ea1 took {:?}", now.elapsed());
    assert_eq!(
        ga1,
        ea1,
        "{}",
        if let (Ok(ga1), Ok(ea1)) = (ga1.parse::<u64>(), ea1.parse::<u64>()) {
            if ga1 > ea1 {
                format!("ga1: {ga1} > ea1: {ea1}")
            } else if ga1 < ea1 {
                format!("ga1: {ga1} < ea1: {ea1}")
            } else {
                unreachable!()
            }
        } else if let (Ok(ga1), Ok(ea1)) = (ga1.parse::<f64>(), ea1.parse::<f64>()) {
            if ga1 > ea1 {
                format!("ga1: {ga1} > ea1: {ea1}")
            } else if ga1 < ea1 {
                format!("ga1: {ga1} < ea1: {ea1}")
            } else {
                unreachable!()
            }
        } else {
            format!("parsing failed")
        }
    );
    println!("ga1: {ga1} == ea1: {ea1}");
    if i1.is_empty() {
        panic!("i1.txt empty dumbass");
    }
    let now = Instant::now();
    let a1 = format!("{:?}", part1(i1));
    println!("--- PART1: {a1}");
    println!("a1 took {:?}\n\n\n", now.elapsed());
    if ca1.is_empty() {
        panic!("save the answer in ca1 before you f up");
    }
    assert_eq!(ca1, a1, "answer differs");

    if e2.is_empty() {
        panic!("e2.txt empty dumbass");
    }
    assert!(!ga2.is_empty(), "ga2 empty dumbass");
    let now = Instant::now();
    let ea2 = format!("{:?}", part2(e2));
    println!("ea2 took {:?}", now.elapsed());
    assert_eq!(
        ga2,
        ea2,
        "{}",
        if let (Ok(ga2), Ok(ea2)) = (ga2.parse::<u64>(), ea2.parse::<u64>()) {
            if ga2 > ea2 {
                format!("ga2: {ga2} > ea2: {ea2}")
            } else if ga2 < ea2 {
                format!("ga2: {ga2} < ea2: {ea2}")
            } else {
                unreachable!()
            }
        } else if let (Ok(ga2), Ok(ea2)) = (ga2.parse::<f64>(), ea2.parse::<f64>()) {
            if ga2 > ea2 {
                format!("ga2: {ga2} > ea2: {ea2}")
            } else if ga2 < ea2 {
                format!("ga2: {ga2} < ea2: {ea2}")
            } else {
                unreachable!()
            }
        } else {
            format!("parsing failed")
        }
    );
    println!("ga2: {ga2} == ea2: {ea2}");
    if i2.is_empty() {
        panic!("i2.txt empty dumbass");
    }
    let now = Instant::now();
    let a2 = format!("{:?}", part2(i2));
    println!("--- PART2: {a2}");
    println!("a2 took {:?}", now.elapsed());
    if ca2.is_empty() {
        panic!("save the answer in ca2 before you f up");
    }
    assert_eq!(ca2, a2, "answer differs");
}

pub fn get_pb(len: usize, msg: String) -> ProgressBar {
    let pb = ProgressBar::new(len as u64);
    let msg: &'static str = msg.clone().leak();

    let pb_style = ProgressStyle::default_bar()
            .template(
                        "{spinner:.green} [{elapsed}] {msg} [{wide_bar:.cyan/blue}] ({pos}/{len}|{percent}%) ({per_sec}|{eta})",
                    )
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(pb_style);
    pb.set_message(msg);
    pb.tick();

    pb
}

#[derive(Debug, Clone, Default)]
struct ParsedInput {
    seeds: Vec<i64>,
    inner: HashMap<(String, String), HashMap<i64, i64>>,
}

#[derive(Debug, Clone, Default)]
struct Inner {
    map: HashMap<Key, Value>,
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    from: String,
    to: String,
}

#[derive(Debug, Clone, Default)]
struct Value {
    inner: Vec<MapChunkRanges>,
}

impl std::str::FromStr for ParsedInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seeds_raw = lines.next().unwrap();
        let len = seeds_raw.len();
        let (_, seeds_raw) = seeds_raw.split_once(":").unwrap();

        let seeds = seeds_raw
            .trim()
            .split(" ")
            .map(|raw_number| raw_number.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let (_, s) = s.split_at(len);

        let inner = parse(s)
            .map
            .into_iter()
            .map(|(Key { from, to }, Value { inner })| {
                let value_map = inner
                    .into_iter()
                    .flat_map(
                        |MapChunkRanges {
                             dst_range_start,
                             src_range_start,
                             range_len,
                         }| {
                            let range_to_add = 0..range_len;
                            range_to_add
                                .into_iter()
                                .map(|v| (dst_range_start + v, src_range_start + v))
                                .collect::<HashMap<i64, i64>>()
                        },
                    )
                    .collect::<HashMap<i64, i64>>();

                ((from.clone(), to.clone()), value_map.clone())
            })
            .collect::<HashMap<_, _>>();
        // HashMap<(String, String), HashMap<i64, i64>>
        Ok(ParsedInput {
            inner,
            seeds,
            ..ParsedInput::default()
        })
    }
}

#[derive(Debug, Clone, Default)]
struct MapChunks {
    from: String,
    to: String,
    ranges: Vec<MapChunkRanges>,
}

#[derive(Debug, Clone, Default)]
struct MapChunkRanges {
    dst_range_start: i64,
    src_range_start: i64,
    range_len: i64,
}

fn parse(input: &str) -> Inner {
    let mut chunks = vec![];
    let last_chunk = input.trim().lines().fold(vec![], |acc, elem| {
        if elem.trim().is_empty() {
            chunks.push(acc.clone());
            vec![]
        } else {
            let mut acc = acc;
            acc.push(elem);
            acc
        }
    });
    chunks.push(last_chunk);

    let map = chunks
        .iter()
        .map(|chunk| {
            let mut lines = chunk.iter();
            let (from, raw_to) = lines.next().unwrap().split_once('-').unwrap();
            let (_, raw_to) = raw_to.split_once('-').unwrap();
            let (to, _) = raw_to.split_once(' ').unwrap();

            let from = from.to_string();
            let to = to.to_string();

            let ranges = lines
                .map(|line| {
                    let (dst_range_start, line) = line.split_once(' ').unwrap();
                    let (src_range_start, range_len) = line.split_once(' ').unwrap();

                    let dst_range_start = dst_range_start.parse().unwrap();
                    let src_range_start = src_range_start.parse().unwrap();
                    let range_len = range_len.parse().unwrap();

                    MapChunkRanges {
                        dst_range_start,
                        src_range_start,
                        range_len,
                    }
                })
                .collect::<Vec<_>>();

            (from, to, ranges)
        })
        .map(|(from, to, map)| (Key { from, to }, map))
        .map(|(key, value)| (key, Value { inner: value }))
        .collect::<HashMap<Key, Value>>();

    Inner { map }

    // #[derive(Debug, Clone, Default)]
    // struct Inner {
    //     maps: HashMap<Key, Value>,
    // }

    // #[derive(Debug, Clone, Default, Hash)]
    // struct Key {
    //     from: String,
    //     to: String,
    // }

    // #[derive(Debug, Clone, Default)]
    // struct Value {
    //     inner: Vec<MapChunkRanges>,
    // }
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part1(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    let ParsedInput { seeds, inner } = input.parse::<ParsedInput>().unwrap();
    seeds
        .into_iter()
        .progress_with(get_pb(lines, format!("part 1 w/ {lines} lines")))
        .min_by_key(|seed| {
            let starting_key = inner.keys().find(|(from, _)| from == "seed").unwrap();

            let mut keys_from = vec![];
            let mut keys_to = vec![];

            inner.keys().for_each(|(from, to)| {
                keys_from.push(from.clone());
                keys_to.push(to.clone());
            });

            let mut maps = vec![];

            for idx in 0..inner.len() {
                let map = inner
                    .get(&(keys_from[idx].clone(), keys_to[idx].clone()))
                    .unwrap()
                    .clone();
                maps.push(map);
            }

            let (froms, tos, maps) = (keys_from, keys_to, maps);

            let starting_idx = froms
                .iter()
                .find_position(|v| v == &&"seed".to_string())
                .unwrap()
                .0;
            // .find("seed".to_string()).unwrap();

            let next_index = |s: String| froms.iter().find_position(|v| v == &&s).unwrap().0;
            let next = (
                next_index(tos[starting_idx].clone()),
                tos[starting_idx].clone(),
                froms
                    .iter()
                    .find_position(|v| v == &&tos[starting_idx])
                    .unwrap()
                    .1
                    .clone(),
                maps[starting_idx].clone(),
            );
            // let find_next_k_to =
            //     |from: String| keys.iter().find(|(k, _)| k == &from).unwrap().1.clone();

            // let find_next_v_to = |from: i64, map: &HashMap<i64, i64>| {
            //     map.get(&from).unwrap_or_else(|| &from).to_owned()
            // };

            // let get_hashmap = |from: String, to: String| inner.get(&(from, to)).unwrap();

            // let (k_prev_from, k_prev_to) = starting_key.clone();
            // // let next_node =

            // let from_seed_to_whatever1 = |seed: i64, map: &HashMap<i64, i64>| {
            //     map.get(&seed).unwrap_or_else(|| &seed).to_owned()
            // };
        })
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part2(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    input
        .parse::<ParsedInput>()
        .unwrap()
        .inner
        .into_iter()
        .progress_with(get_pb(lines, format!("part 2 w/ {lines} lines")))
        .collect::<Vec<_>>()
}
