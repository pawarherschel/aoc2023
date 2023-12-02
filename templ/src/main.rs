#![allow(unused)]

fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt").trim();
    let e1 = include_str!("e1.txt").trim();
    let ga1 = "";
    let ga1 = ga1.to_string();

    let i2 = include_str!("i2.txt").trim();
    let e2 = include_str!("e2.txt").trim();
    let ga2 = "";
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
    let ca1 = "";
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
    let ca2 = "";
    let ca2 = ca2.to_string();
    if ca2.is_empty() {
        panic!("save the answer in ca2 before you f up");
    }
    assert_eq!(ca2, a2);
}

fn part1(input: &str) -> String {
    "unimplemented!(\"part 1\")".to_string()
}

fn part2(input: &str) -> String {
    "unimplemented!(\"part 2\")".to_string()
}
