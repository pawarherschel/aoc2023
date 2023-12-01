fn main() {
    // i: input
    // e: example
    // g: given
    // a: answer
    // c: calculated

    let i1 = include_str!("i1.txt");
    let e1 = include_str!("e1.txt");
    let ga1 = "";
    let ga1 = ga1.to_string();

    let i2 = include_str!("i2.txt");
    let e2 = include_str!("e2.txt");
    let ga2 = "";
    let ga2 = ga2.to_string();

    let ea1 = part1(e1);
    assert_eq!(ga1, ea1);
    let a1 = part1(i1);
    println!("--- PART1: {a1}");
    let ca1 = "";
    let ca1 = ca1.to_string();
    assert_eq!(ca1, a1);

    let ea2 = part2(e2);
    assert_eq!(ga2, ea2);
    let a2 = part2(i2);
    println!("--- PART2: {a2}");
    let ca2 = "";
    let ca2 = ca2.to_string();
    assert_eq!(ca2, a2);
}

fn part1(input: &str) -> String {
    unimplemented!("part 1")
}

fn part2(input: &str) -> String {
    unimplemented!("part 2")
}
