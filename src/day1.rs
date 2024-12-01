use itertools::Itertools;

const INPUT: &str = include_str!("./day1.txt");

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in INPUT.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        vec1.push(a);
        vec2.push(b);
    }

    (vec1, vec2)
}

fn part1() -> i32 {
    let (mut a, mut b) = get_input();

    a.sort();
    b.sort();

    let differences = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    differences
}

fn part2() -> i32 {
    let (a, b) = get_input();

    let counts = b.into_iter().counts();

    let result = a
        .into_iter()
        .map(|x| x * counts.get(&x).copied().unwrap_or(0) as i32)
        .sum::<i32>();

    result
}

pub fn solve() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
