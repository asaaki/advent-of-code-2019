// https://adventofcode.com/2019/day/3

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::Read;

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

    Ok(())
}

fn read_data(mut file: File) -> String {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tbd() {
        assert!(true);
    }

    // R75,D30,R83,U83,L12,D49,R71,U7,L72
    // U62,R66,U55,R34,D71,R55,D58,R83
    // distance 159
    #[test]
    fn test_part1_example1() {}

    // R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    // U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
    // distance 135
    #[test]
    fn test_part1_example2() {}
}
