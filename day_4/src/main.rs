// https://adventofcode.com/2019/day/4

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::Read;

type Code = isize;
type CodeRange = std::ops::RangeInclusive<Code>;
type Passwords = Vec<Code>;

fn main() -> std::io::Result<()> {
    let clap_app = App::new("AoC day 4")
        .version("1.0")
        .about("Count the passwords")
        .author("@asaaki")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        );
    let matches = clap_app.get_matches();

    let input_file_path = fs::canonicalize(matches.value_of("INPUT").unwrap())?;
    let file = File::open(&input_file_path)?;
    let pass_range = read_data(file);
    let passwords = calculate_passwords(pass_range);
    let normal_length = passwords.len();
    let strict_passwords = calculate_passwords_strict(passwords);

    println!("Part 1: {}", normal_length);
    println!("Part 2: {}", strict_passwords.len());

    Ok(())
}

fn read_data(mut file: File) -> CodeRange {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let data: Vec<Code> = buffer
        .trim()
        .split('-')
        .map(|s| s.parse::<Code>().unwrap())
        .collect();
    data[0]..=data[1]
}

fn calculate_passwords(pass_range: CodeRange) -> Passwords {
    let mut passwords: Passwords = vec![];
    for potential in pass_range {
        if is_sorted(potential) && has_adjacent_doubles(potential) {
            passwords.push(potential)
        }
    }
    passwords
}

fn calculate_passwords_strict(loose_phrases: Passwords) -> Passwords {
    loose_phrases
        .iter()
        .cloned()
        .filter(|p| at_least_one_pair(*p))
        .collect()
}

// integer-only sorted digit check
fn is_sorted(input: Code) -> bool {
    let mut potential = input;
    let mut digits: Vec<Code> = vec![];
    while potential > 0 {
        let part = potential % 10;
        digits.push(part);
        potential /= 10;
    }
    digits.reverse();
    let mut sorted = digits.clone();
    sorted.sort();
    digits == sorted
}

fn has_adjacent_doubles(input: Code) -> bool {
    let mut chars = int2vec(input);
    chars.dedup();
    chars.len() < 6
}

fn int2vec(input: Code) -> Vec<String> {
    string2vec(format!("{}", input))
}

fn string2vec(input: String) -> Vec<String> {
    input.chars().map(|c| c.to_string()).collect()
}

fn at_least_one_pair(input: Code) -> bool {
    let chars = int2vec(input);
    let mut m = std::collections::HashMap::new();
    for c in chars {
        *m.entry(c).or_insert(0) += 1
    }
    let counts: Vec<u32> = m.values().cloned().collect();
    counts.contains(&2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example0() {
        let passwords = calculate_passwords(111_110..=223_456);
        assert_eq!(passwords.contains(&111_111), true);
        assert_eq!(passwords.contains(&223_450), false);
        assert_eq!(passwords.contains(&123_789), false);
    }

    #[test]
    fn test_part2_example0() {
        let passwords = calculate_passwords(111_110..=223_456);
        let strict = calculate_passwords_strict(passwords);
        assert_eq!(strict.contains(&112_233), true);
        assert_eq!(strict.contains(&123_444), false);
        assert_eq!(strict.contains(&111_122), true);
    }
}
