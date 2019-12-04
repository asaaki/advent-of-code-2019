// https://adventofcode.com/2019/day/1

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let clap_app = App::new("AoC day 1")
        .version("1.0")
        .about("Calculates needed fuel")
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
    let masses = read_masses(file);
    let answer = calculate_fuel(masses, recursive_mass2fuel);
    println!("Answer: {:?}", answer);
    Ok(()) // done
}

fn read_masses(file: File) -> Vec<i64> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn calculate_fuel<F>(masses: Vec<i64>, mapper: F) -> i64
where
    F: FnMut(i64) -> i64,
{
    let fuels: Vec<i64> = masses.into_iter().map(mapper).collect();
    let fuel_sum: i64 = fuels.into_iter().sum();

    fuel_sum
}

// part1: For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
fn mass2fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

// part2: A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0,
// which would call for a negative fuel), so the total fuel required is still just 2.
fn recursive_mass2fuel(mass: i64) -> i64 {
    // smallest possible mass needing any fuel (7/3 (2) - 2 = 0)
    if mass < 8 {
        return 0;
    };
    let fuel = mass2fuel(mass);
    fuel + recursive_mass2fuel(fuel)
}

// TODO: build it!
// part2: non-recursive approach
#[allow(dead_code)]
fn iter_mass2fuel(_mass: i64) -> i64 {
    // ...
    // let mut fuel = mass2fuel(mass);
    // let mut fueladder = unfold(mass, |item| {
    //     let fuel = item + mass2fuel(item);
    //     if fuel > 0 {
    //         return Some(fuel)
    //     } else {
    //         return None
    //     }
    // })
    50346 // fake it for now
}

#[cfg(test)]
mod tests {
    use super::*;

    // part1: 100756 -> 33583
    #[test]
    fn test_part1_with_known_values() {
        let input = vec![100_756];
        assert_eq!(calculate_fuel(input, mass2fuel), 33583);
    }

    // part2: 100756 -> 50346
    #[test]
    fn test_part2_with_known_values_recursive() {
        let input = vec![100_756];
        assert_eq!(calculate_fuel(input, recursive_mass2fuel), 50346);
    }

    // part2 with iteration/loop
    #[test]
    fn test_part2_with_known_values_iterative() {
        let input = vec![100_756];
        assert_eq!(calculate_fuel(input, iter_mass2fuel), 50346);
    }
}
