use std::time::{Duration, Instant};

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let start = Instant::now();

    for _ in 0..10_000 {
        let i1 = include_str!("i1.txt");
        let e1 = include_str!("e1.txt");
        let ga1 = "8";
        let ga1 = ga1.to_string();

        let i2 = include_str!("i2.txt");
        let e2 = include_str!("e2.txt");
        let ga2 = "2286";
        let ga2 = ga2.to_string();

        let ea1 = part1(e1);
        assert_eq!(ga1, ea1);
        let a1 = part1(i1);
        println!("--- PART1: {a1}");
        let ca1 = "2331";
        let ca1 = ca1.to_string();
        assert_eq!(ca1, a1);

        let ea2 = part2(e2);
        assert_eq!(ga2, ea2);
        let a2 = part2(i2);
        println!("--- PART2: {a2}");
        let ca2 = "71585";
        let ca2 = ca2.to_string();
        assert_eq!(ca2, a2);
    }

    let time = Duration::from_secs_f64(start.elapsed().as_secs_f64() / 10_000.0);
    dbg!(time);
}

#[derive(Clone, Default, Debug)]
struct Die {
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

#[derive(Clone, Default, Debug)]
struct DieFin {
    red: u32,
    green: u32,
    blue: u32,
}

static EXAMPLE_01_FILTER: DieFin = DieFin {
    red: 12,
    green: 13,
    blue: 14,
};

fn part1(input: &str) -> String {
    let state = input.lines().map(|line| {
        let (id_raw, subsets_raw) = line.split_once(':').unwrap();
        let id = id_raw.split_once(' ').unwrap().1.parse::<u32>().unwrap();

        let subsets = subsets_raw.split(';').collect::<Vec<&str>>();
        let subsets_with_dies_raw = subsets
            .into_iter()
            .map(|it| it.split(',').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let subsets_with_dies = subsets_with_dies_raw
            .into_iter()
            .map(|it| {
                it.into_iter()
                    .map(|it| it.trim().split_once(' ').unwrap())
                    .map(|(num_raw, die_type)| (num_raw.parse::<u32>().unwrap(), die_type))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut game = Die::default();
        for subset in subsets_with_dies {
            for (num, die) in subset {
                match die {
                    "red" => game.red.push(num),
                    "green" => game.green.push(num),
                    "blue" => game.blue.push(num),
                    _ => panic!("{}", format!("unknown: ({}, {}) w/ {:?}", num, die, game)),
                }
            }
        }
        (id, game)
    });
    state
        .into_iter()
        .filter(|(_, die)| {
            let Die { red, green, blue } = die;
            !(red.iter().any(|it| it > &EXAMPLE_01_FILTER.red)
                || green.iter().any(|it| it > &EXAMPLE_01_FILTER.green)
                || blue.iter().any(|it| it > &EXAMPLE_01_FILTER.blue))
        })
        .map(|(it, _)| it)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let state = input.lines().map(|line| {
        let (id_raw, subsets_raw) = line.split_once(':').unwrap();
        let id = id_raw.split_once(' ').unwrap().1.parse::<u32>().unwrap();

        let subsets = subsets_raw.split(';').collect::<Vec<&str>>();
        let subsets_with_dies_raw = subsets
            .into_iter()
            .map(|it| it.split(',').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let subsets_with_dies = subsets_with_dies_raw
            .into_iter()
            .map(|it| {
                it.into_iter()
                    .map(|it| it.trim().split_once(' ').unwrap())
                    .map(|(num_raw, die_type)| (num_raw.parse::<u32>().unwrap(), die_type))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut game = Die::default();
        for subset in subsets_with_dies {
            for (num, die) in subset {
                match die {
                    "red" => game.red.push(num),
                    "green" => game.green.push(num),
                    "blue" => game.blue.push(num),
                    _ => panic!("{}", format!("unknown: ({}, {}) w/ {:?}", num, die, game)),
                }
            }
        }
        (id, game)
    });

    state
        .into_iter()
        .map(|(_, dies)| {
            let Die { red, green, blue } = dies;

            let red = red.into_iter().max().unwrap();
            let green = green.into_iter().max().unwrap();
            let blue = blue.into_iter().max().unwrap();

            DieFin { red, green, blue }
        })
        .map(|it| {
            let DieFin { red, green, blue } = it;
            red * green * blue
        })
        .sum::<u32>()
        .to_string()
}
