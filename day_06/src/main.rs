#![allow(unused)]

use indicatif::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "288";
    let ga1 = ga1.to_string();
    let ca1 = "303600";
    let ca1 = ca1.to_string();

    let i2 = include_str!("i2.txt").trim();
    let e2 = include_str!("e2.txt").trim();
    let ga2 = "71503";
    let ga2 = ga2.to_string();
    let ca2 = "23654842";
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
    inner: Vec<Inner>,
}

#[derive(Debug, Clone, Default)]
struct Inner {
    time: u64,
    distance: u64,
}

impl std::str::FromStr for ParsedInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ParsedInput {
            inner: parse(s),
            ..ParsedInput::default()
        })
    }
}

fn parse(input: &str) -> Vec<Inner> {
    let (raw_time, raw_distance) = input.trim().split_once("\n").unwrap();
    let raw_time = raw_time.trim();
    let (_, raw_time) = raw_time.split_once(":").unwrap();
    let time = raw_time
        .trim()
        .split_whitespace()
        .map(|it| it.parse::<u64>().unwrap());

    let raw_distance = raw_distance.trim();
    let (_, raw_distance) = raw_distance.split_once(":").unwrap();
    let distance = raw_distance
        .trim()
        .split_whitespace()
        .map(|it| it.parse::<u64>().unwrap());

    time.zip(distance)
        .map(|(time, distance)| Inner { time, distance })
        .collect()
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part1(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    input
        .parse::<ParsedInput>()
        .unwrap()
        .inner
        .into_iter()
        .progress_with(get_pb(lines, format!("part 1 w/ {lines} lines")))
        .map(|Inner { time, distance }| {
            (0..=time)
                .filter_map(|vel_time| {
                    let non_vel_time = time - vel_time;
                    let velocity = vel_time;
                    let distance_travelled = velocity * non_vel_time;

                    if distance_travelled > distance {
                        Some(vel_time)
                    } else {
                        None
                    }
                })
                .count() as u64
        })
        .product::<u64>()
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part2(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    let (time, distance) = input
        .parse::<ParsedInput>()
        .unwrap()
        .inner
        .into_iter()
        .progress_with(get_pb(lines, format!("part 2 w/ {lines} lines")))
        .map(|Inner { time, distance }| (time, distance))
        .map(|(t, d)| (t.to_string(), d.to_string()))
        .unzip::<String, String, Vec<String>, Vec<String>>();
    let time = time.join("").parse::<u64>().unwrap();
    let distance = distance.join("").parse::<u64>().unwrap();
    (0..=time)
        .filter_map(|vel_time| {
            let non_vel_time = time - vel_time;
            let velocity = vel_time;
            let distance_travelled = velocity * non_vel_time;

            if distance_travelled > distance {
                Some(vel_time)
            } else {
                None
            }
        })
        .count() as u64
}
