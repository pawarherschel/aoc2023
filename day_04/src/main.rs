#![allow(unused)]

use std::{collections::HashMap, ops::Deref};

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "13";
    let ga1 = ga1.to_string();

    let i2 = include_str!("i2.txt").trim();
    let e2 = include_str!("e2.txt").trim();
    let ga2 = "30";
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
    let ca1 = "21213";
    let ca1 = ca1.to_string();
    if ca1.is_empty() {
        panic!("save the answer in ca1 before you f up");
    }
    assert_eq!(ca1, a1);

    if e2.is_empty() {
        panic!("e2.txt empty dumbass");
    }
    let ea2 = format!("{:?}", part2(e2));
    assert_ne!(ga2, "", "ga2 empty dumbass");
    assert_eq!(ga2, ea2);
    if i2.is_empty() {
        panic!("i2.txt empty dumbass");
    }
    let a2 = format!("{:?}", part2(i2));
    println!("--- PART2: {a2:?}");
    let ca2 = "8549735";
    let ca2 = ca2.to_string();
    if ca2.is_empty() {
        panic!("save the answer in ca2 before you f up");
    }
    assert_eq!(ca2, a2);
}

#[derive(Clone, Debug)]
struct Game {
    number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (raw_card_number, raw_card_details) = line.split_once(':').unwrap();
            let (_, raw_number) = raw_card_number.split_once(' ').unwrap();
            let number = raw_number
                .trim()
                .parse()
                .unwrap_or_else(|a| panic!("{}", raw_number));

            let (raw_winning_numbers, raw_my_numbers) = raw_card_details.split_once('|').unwrap();
            let winning_numbers = raw_winning_numbers
                .split_whitespace()
                .map(|it| it.parse().unwrap())
                .collect();
            let my_numbers = raw_my_numbers
                .split_whitespace()
                .map(|it| it.parse().unwrap())
                .collect();

            Game {
                number,
                winning_numbers,
                my_numbers,
            }
        })
        .collect()
}

fn part1(input: &str) -> String {
    let games = parse(input);
    games
        .into_iter()
        .map(|game| {
            // println!("game: {}", &game.number);
            game.my_numbers
                .iter()
                .cloned()
                .filter(|it| {
                    game.winning_numbers
                        .clone()
                        .into_iter()
                        .any(|win| win == *it)
                })
                .collect::<Vec<_>>()
        })
        // .inspect(|it| println!("winning numbers: {:?}", it))
        .filter_map(|it| {
            if !it.is_empty() {
                Some(2_u32.pow((it.len() as u32 - 1)))
            } else {
                None
            }
        })
        // .inspect(|it| println!("points: {it}"))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> impl std::fmt::Debug {
    let mut accesses = Box::new(RefCell::new(HashMap::new()));

    let mut cache: Box<RefCell<HashMap<u32, Vec<u32>>>> = Box::new(RefCell::new(HashMap::new()));

    let _games = parse(input);

    use std::cell::RefCell;

    let mut game_copies = Box::new(RefCell::new(vec![]));

    let _max = _games
        .iter()
        .max_by(|a, b| a.number.cmp(&b.number))
        .unwrap()
        .number as usize;

    // // println!("{:?}", (accesses, _games, game_copies, max));

    let x = _games
        .clone()
        .into_iter()
        .map(|game| {
            println!("processing game: {}", &game.number);
            accesses
                .borrow_mut()
                .entry(game.number)
                .and_modify(|it: &mut u32| *it += 1_u32)
                .or_insert(1);
            let mut i = (1..);
            let copies = match cache.borrow().get(&game.number) {
                Some(hit) => hit.clone(),
                None => game
                    .winning_numbers
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(_, &it)| game.my_numbers.clone().iter().any(|&num| it == num))
                    .map(|(a, b)| (a.to_owned(), b.clone()))
                    .map(|(a, b)| game.number.clone() as u32 + i.next().unwrap())
                    .collect::<Vec<_>>(),
            };
            for c in copies.iter() {
                let index = *c as usize - 1;
                //             println!("max here: {:?}", (index, _max));
                if index < _max {
                    let game_to_push = _games.get(index).unwrap().clone();
                    //                 println!("pushing game number: {}", &game_to_push.number);
                    game_copies.borrow_mut().push(game_to_push);
                }
            }
            cache.borrow_mut().insert(game.number, copies.clone());
            (game.number, copies)
        })
        // .inspect(|it| {
        //         eprintln!(
        // "it {:?}\ngame_copies: {:?}",
        // it,
        /*games.get(*it as usize).unwrap()*/
        // &game_copies
        // .borrow()
        // .iter()
        // .map(|it| it.number)
        // .collect::<Vec<_>>()
        // )
        // })
        .flat_map(|(it, _)| {
            let g = game_copies.borrow().clone();
            g.iter()
                .filter(|&Game { number, .. }| *number == it)
                .flat_map(|game| {
                    accesses
                        .borrow_mut()
                        .entry(game.number)
                        .and_modify(|it| *it += 1);
                    //                 println!("game (copy): {}", &game.number);
                    let mut i = (1..);
                    let copies = cache.borrow().get(&game.number).unwrap().clone();
                    for c in copies.iter() {
                        let index = *c as usize - 1;
                        //                     println!("max here: {:?}", (index, _max));
                        if index < _max {
                            let game_to_push = _games.get(index).unwrap().clone();
                            //                         println!("pushing game number: {}", &game_to_push.number);
                            game_copies.borrow_mut().push(game_to_push);
                        }
                    }
                    copies.into_iter().collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        //     .inspect(|_| println!("accesses: {:?}", accesses.borrow().clone()))
        .collect::<Vec<_>>();

    // println!("{:?}", &x);
    let x = accesses;
    // println!("{:?}", &x);
    let x = x.borrow();
    // println!("{:?}", &x);
    let x = x.clone();
    // println!("{:?}", &x);
    let x = x.iter();
    // println!("{:?}", &x);
    // let x = x.inspect(|it| println!("{:?}", it));
    // println!("{:?}", &x);
    let x = x.map(|(k, v)| *v);
    // println!("{:?}", &x);
    let x = x.sum::<u32>();
    // println!("{:?}", &x);
    x
}
