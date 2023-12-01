fn main() {
    let i1 = include_str!("./input1.txt");
    let ie1 = include_str!("./example1.txt");

    let example_01 = "142".to_string();
    assert_eq!(example_01, part1(ie1));
    println!("part 1: {}", part1(i1));
    assert_eq!("54667".to_string(), part1(i1));

    let i2 = include_str!("./input2.txt");
    let ie2 = include_str!("./example2.txt");

    let example_02 = "281".to_string();
    assert_eq!(example_02, part2(ie2));
    println!("part 2: {}", part2(i2));
    assert_eq!("54203".to_string(), part2(i2));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|it| it.is_digit(10))
                .map(|it| it.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| {
            (
                digits.clone().first().unwrap().clone(),
                digits.last().unwrap().clone(),
            )
        })
        .map(|(a, b)| a * 10 + b)
        .sum::<u32>()
        .to_string()
}

const DIGITS: [(&str, &str); 9] = [
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

fn part2(input: &str) -> String {
    part1(
        input
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                for digit in DIGITS {
                    line = line.replace(digit.0, digit.1);
                }
                line
            })
            .collect::<Vec<String>>()
            .join("\n")
            .as_str(),
    )
}
