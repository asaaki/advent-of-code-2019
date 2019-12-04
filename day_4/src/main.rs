// https://adventofcode.com/2019/day/3

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::Read;

type Code = isize;
type CodeRange = std::ops::RangeInclusive<Code>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passphrase {
    code: Code,
}

type Passphrases = Vec<Passphrase>;

fn main() -> std::io::Result<()> {
    let clap_app = App::new("AoC day 3")
        .version("1.0")
        .about("Find the distance")
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
    let pass_phrases = calculate_passphrases(pass_range);
    let strict_pass_phrases = calculate_passphrases_strict(pass_phrases.clone());

    println!("Part 1: {}", pass_phrases.len());
    println!("Part 2: {}", strict_pass_phrases.len());

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

fn calculate_passphrases(pass_range: CodeRange) -> Passphrases {
    let mut passphrases: Passphrases = vec![];
    for potential in pass_range {
        let p_string = format!("{}", potential);
        let mut chars: Vec<String> = p_string.chars().map(|c| format!("{}", c)).collect();
        let sorted: Code = {
            let mut sc = chars.clone();
            sc.sort();
            let sc1: String = sc.into_iter().collect();
            sc1.parse().unwrap()
        };

        let is_sorted = potential == sorted;

        // if there are at least 2 adjacent duplicates, the deduped version is shorter
        chars.dedup();
        let has_adjacent_doubles = chars.len() < 6;

        if is_sorted && has_adjacent_doubles {
            passphrases.push(Passphrase { code: potential })
        }
    }
    passphrases
}

fn calculate_passphrases_strict(loose_phrases: Passphrases) -> Passphrases {
    loose_phrases
        .iter()
        .cloned()
        .filter(|p| {
            let p_string = format!("{}", p.code);
            let chars: Vec<String> = p_string.chars().map(|c| format!("{}", c)).collect();
            let mut m = std::collections::HashMap::new();
            for c in chars {
                *m.entry(c).or_insert(0) += 1
            }
            let counts: Vec<u32> = m.values().cloned().collect();
            counts.contains(&2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example0() {
        let pass_phrases = calculate_passphrases(111_110..=223_456);
        assert_eq!(pass_phrases.contains(&Passphrase { code: 111_111 }), true);
        assert_eq!(pass_phrases.contains(&Passphrase { code: 223_450 }), false);
        assert_eq!(pass_phrases.contains(&Passphrase { code: 123_789 }), false);
    }

    #[test]
    fn test_part2_example0() {
        let pass_phrases = calculate_passphrases(111_110..=223_456);
        let strict = calculate_passphrases_strict(pass_phrases);
        assert_eq!(strict.contains(&Passphrase { code: 112_233 }), true);
        assert_eq!(strict.contains(&Passphrase { code: 123_444 }), false);
        assert_eq!(strict.contains(&Passphrase { code: 111_122 }), true);
    }
}
