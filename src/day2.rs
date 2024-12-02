use itertools::Itertools;

const INPUT: &str = include_str!("./day2.txt");

fn get_input() -> Vec<Vec<i32>> {
    INPUT
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| i32::from_str_radix(x, 10).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe(line: &[i32]) -> bool {
    let is_ascending = line.iter().is_sorted();
    let is_descending = line.iter().rev().is_sorted();

    let enough_difference = line.iter().tuple_windows().all(|(a, b)| {
        let diff = (a - b).abs();
        diff >= 1 && diff <= 3
    });

    (is_ascending || is_descending) && enough_difference
}

pub fn part1() -> i32 {
    let input = get_input();
    let safe_count = input.iter().filter(|v| is_safe(v.as_slice())).count();
    safe_count as i32
}

pub fn part2() -> i32 {
    let input = get_input();

    let safe_count = input.iter().filter(|v| {
        (0..v.len())
            .map(|i| {
                let mut v = (*v).clone();
                v.remove(i);
                v
            })
            .any(|v| is_safe(v.as_slice()))
    });

    safe_count.count() as i32
}

pub fn solve() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
