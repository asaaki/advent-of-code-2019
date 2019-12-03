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
    let answer = calculate_fuel(masses);
    println!("Answer: {:?}", answer);
    Ok(()) // done
}

fn read_masses(file: File) -> Vec<i64> {
    let reader = BufReader::new(file);
    let mut masses: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let mass = line.unwrap().parse().unwrap();
        masses.push(mass);
    }
    masses
}

fn calculate_fuel(masses: Vec<i64>) -> i64 {
    let fuels: Vec<i64> = masses.into_iter().map(mass2fuel).collect();
    let fuel_sum: i64 = fuels.into_iter().sum();

    fuel_sum
}

// part1: For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
// part2: A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0,
// which would call for a negative fuel), so the total fuel required is still just 2.
fn mass2fuel(mass: i64) -> i64 {
    // smallest possible mass needing any fuel (7/3 (2) - 2 = 0)
    if mass < 8 {
        return 0;
    };
    let fuel = (mass / 3) - 2;
    fuel + mass2fuel(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    // part2: 100756 -> 50346
    #[test]
    fn test_with_known_values() {
        let input = vec![100756];
        assert_eq!(calculate_fuel(input), 50346);
    }
}
