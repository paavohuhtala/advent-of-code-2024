use regex::Regex;

const INPUT: &str = include_str!("./day3.txt");

const MUL_REGEX: &str = r"mul\((\d+),(\d+)\)";
const DO_REGEX: &str = r"do\(\)";
const DONT_REGEX: &str = r"don't\(\)";

enum Intruction {
    Mul(i32, i32),
    SetMulEnabled(bool),
}

fn get_input() -> Vec<Intruction> {
    let any_instruction_regex = format!("(?:{}|{}|{})", MUL_REGEX, DO_REGEX, DONT_REGEX);
    let re = Regex::new(&any_instruction_regex).unwrap();
    let mut vec = Vec::new();

    for m in re.captures_iter(INPUT) {
        let instruction = match &m[0] {
            "do()" => Intruction::SetMulEnabled(true),
            "don't()" => Intruction::SetMulEnabled(false),
            _ => {
                let a = m[1].parse().unwrap();
                let b = m[2].parse().unwrap();

                Intruction::Mul(a, b)
            }
        };

        vec.push(instruction);
    }

    vec
}

fn part1() -> i32 {
    let instructions = get_input();

    instructions
        .iter()
        .map(|instruction| match instruction {
            Intruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn part2() -> i32 {
    let instructions = get_input();

    let mut mul_enabled = true;
    let mut total = 0;

    for instruction in instructions {
        match instruction {
            Intruction::Mul(a, b) => {
                if mul_enabled {
                    total += a * b;
                }
            }
            Intruction::SetMulEnabled(enabled) => {
                mul_enabled = enabled;
            }
        }
    }

    total
}

pub fn solve() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
