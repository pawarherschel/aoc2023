#![allow(unused)]
#![feature(array_windows)]

use indicatif::*;
use rayon::prelude::*;
use std::{fmt::Display, time::Instant};

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "6440";
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
    println!("a1 took {:?}\n\n\n", now.elapsed());
    println!("--- PART1: {a1}");
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
    println!("a2 took {:?}", now.elapsed());
    println!("--- PART2: {a2}");
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Card::A => "A",
            Card::K => "K",
            Card::Q => "Q",
            Card::J => "J",
            Card::T => "T",
            Card::_9 => "9",
            Card::_8 => "8",
            Card::_7 => "7",
            Card::_6 => "6",
            Card::_5 => "5",
            Card::_4 => "4",
            Card::_3 => "3",
            Card::_2 => "2",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum Hand {
    FiveOfAKind(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

#[derive(Debug, Clone, Default)]
struct ParsedInput {
    inner: Vec<Inner>,
}

// #[derive(Debug, Clone, Default)]
type Inner = (Hand, u64);

impl std::str::FromStr for ParsedInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ParsedInput {
            inner: parse(s),
            ..ParsedInput::default()
        })
    }
}

#[derive(Debug, Default)]
struct Counts {
    _2: u64,
    _3: u64,
    _4: u64,
    _5: u64,
    _6: u64,
    _7: u64,
    _8: u64,
    _9: u64,
    a: u64,
    j: u64,
    k: u64,
    q: u64,
    t: u64,
}

impl Into<Vec<(Card, u64)>> for Counts {
    fn into(self) -> Vec<(Card, u64)> {
        vec![
            (Card::_2, self._2),
            (Card::_3, self._3),
            (Card::_4, self._4),
            (Card::_5, self._5),
            (Card::_6, self._6),
            (Card::_7, self._7),
            (Card::_8, self._8),
            (Card::_9, self._9),
            (Card::A, self.a),
            (Card::J, self.j),
            (Card::K, self.k),
            (Card::Q, self.q),
            (Card::T, self.t),
        ]
    }
    // fn into(self) -> Vec<u64> {
    //     vec![
    //         self._2, self._3, self._4, self._5, self._6, self._7, self._8, self._9, self.a, self.j,
    //         self.k, self.q, self.t,
    //     ]
    // }
}

fn parse(input: &str) -> Vec<(Hand, u64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (hand_raw, bid_raw) = line.trim().split_once(' ').unwrap();
            let bid = bid_raw.trim().parse::<u64>().unwrap();
            let hand_raw = hand_raw.to_uppercase();
            let hand_raw_vec = hand_raw
                .trim()
                .chars()
                .map(|it| match it {
                    '9' => Card::_9,
                    '8' => Card::_8,
                    '7' => Card::_7,
                    '6' => Card::_6,
                    '5' => Card::_5,
                    '4' => Card::_4,
                    '3' => Card::_3,
                    '2' => Card::_2,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    'A' => Card::A,
                    c => {
                        eprintln!("{c:?}");
                        unreachable!();
                    }
                })
                .collect::<Vec<_>>();

            let hand = {
                let &[a, b, c, d, e] = hand_raw_vec.array_windows::<5>().next().unwrap();
                let counts =
                    vec![a, b, c, d, e]
                        .iter()
                        .fold(Counts::default(), |acc, it| match it {
                            Card::K => Counts {
                                k: acc.k + 1,
                                ..acc
                            },
                            Card::A => Counts {
                                a: acc.a + 1,
                                ..acc
                            },
                            Card::Q => Counts {
                                q: acc.q + 1,
                                ..acc
                            },
                            Card::J => Counts {
                                j: acc.j + 1,
                                ..acc
                            },
                            Card::T => Counts {
                                t: acc.t + 1,
                                ..acc
                            },
                            Card::_9 => Counts {
                                _9: acc._9 + 1,
                                ..acc
                            },
                            Card::_8 => Counts {
                                _8: acc._8 + 1,
                                ..acc
                            },
                            Card::_7 => Counts {
                                _7: acc._7 + 1,
                                ..acc
                            },
                            Card::_6 => Counts {
                                _6: acc._6 + 1,
                                ..acc
                            },
                            Card::_5 => Counts {
                                _5: acc._5 + 1,
                                ..acc
                            },
                            Card::_4 => Counts {
                                _4: acc._4 + 1,
                                ..acc
                            },
                            Card::_3 => Counts {
                                _3: acc._3 + 1,
                                ..acc
                            },
                            Card::_2 => Counts {
                                _2: acc._2 + 1,
                                ..acc
                            },
                        });
                let counts_vec: Vec<(Card, u64)> = counts.into();
                println!("{:?}", counts_vec);

                if let Some((card, _)) = counts_vec.iter().filter(|(_, count)| *count == 5).next() {
                    todo!()
                }
                todo!()
            };

            (hand, bid)
        })
        .inspect(|it| println!("{it:?}"))
        .collect::<Vec<_>>();

    todo!()
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part1(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    input
        .parse::<ParsedInput>()
        .unwrap()
        .inner
        .into_par_iter()
        .progress_with(get_pb(lines, format!("part 1 w/ {lines} lines")))
        .collect::<Vec<_>>()
}

// REMEMBER THAT IF THE ANSWER DEPENDS ON PREVIOUS ITERATIONS THEN YOU CANT USE PAR ITER
fn part2(input: &str) -> impl std::fmt::Debug {
    let lines = input.lines().count();
    input
        .parse::<ParsedInput>()
        .unwrap()
        .inner
        .into_par_iter()
        .progress_with(get_pb(lines, format!("part 2 w/ {lines} lines")))
        .collect::<Vec<_>>()
}
