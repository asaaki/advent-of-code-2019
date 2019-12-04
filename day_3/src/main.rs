// https://adventofcode.com/2019/day/3

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = isize;
type Distance = isize;
type Distances = Vec<Distance>;

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

type LineOp = (Coord, Orientation);
type LineOps = Vec<LineOp>;

#[derive(Debug, Clone)]
struct Point {
    x: Coord,
    y: Coord,
}

type Steps = isize;

#[derive(Debug, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
    last_steps: Steps,
}

type Segments = Vec<Segment>;
type SegmentIndices = Vec<usize>;

#[derive(Debug, Clone, Default)]
struct Line {
    pub segments: Segments,
    // IDs of segment steps for both orientations:
    pub horizontals: SegmentIndices,
    pub verticals: SegmentIndices,
}

#[derive(Debug, Clone)]
struct Intersection {
    point: Point,
    cost: Steps,
}

type Intersections = Vec<Intersection>;

const CENTRAL_PORT: Point = Point { x: 0, y: 0 };

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
    let (ls1, ls2) = read_lines(file);
    let (wire1, wire2) = string2lines(ls1, ls2);

    let shortest_distance = get_shortest_distance(wire1.clone(), wire2.clone()); // sorryNotSorry
    let shortest_steps = get_shortest_steps(wire1, wire2);

    println!("Shortest distance: {:?}", shortest_distance);
    println!("Fewest steps: {:?}", shortest_steps);

    Ok(())
}

fn read_lines(file: File) -> (String, String) {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    (lines[0].clone(), lines[1].clone())
}

fn string2lines(s1: String, s2: String) -> (Line, Line) {
    let (wire1, wire2) = (string2line(s1), string2line(s2));
    (wire1, wire2)
}

fn string2line(input: String) -> Line {
    calculate_line(CENTRAL_PORT.clone(), parse_line_string(input))
}

fn parse_line_string(line_string: String) -> LineOps {
    use Orientation::*;

    line_string
        .trim()
        .split(',')
        .map(|s| {
            let (direction, length_value) = s.split_at(1);
            let length_abs: Coord = length_value.parse().unwrap();
            match direction {
                "L" => (length_abs * -1, Horizontal),
                "R" => (length_abs * 1, Horizontal),
                "U" => (length_abs * 1, Vertical),
                "D" => (length_abs * -1, Vertical),
                d => panic!("Invalid direction encountered: {}", d),
            }
        })
        .collect()
}

fn calculate_line(start: Point, line_ops: LineOps) -> Line {
    use Orientation::*;

    let mut next_point = start;
    let mut last_steps = 0;
    let mut line = Line::default();
    for (pos, (coord, orientation)) in line_ops.iter().enumerate() {
        let point = match orientation {
            Horizontal => Point {
                x: next_point.x.clone() + coord,
                y: next_point.y.clone(),
            },
            Vertical => Point {
                x: next_point.x.clone(),
                y: next_point.y.clone() + coord,
            },
        };
        let segment = Segment {
            p1: next_point,
            p2: point.clone(),
            last_steps: last_steps.clone(),
        };
        next_point = point.clone();
        last_steps += coord.abs();
        line.segments.push(segment);
        match orientation {
            Horizontal => line.horizontals.push(pos.clone()),
            Vertical => line.verticals.push(pos.clone()),
        }
    }
    line
}

// yes, that is not very memory efficient,
// but I also do not want to deal with lifetimes yet
fn get_directed_segments(line: Line) -> (Segments, Segments) {
    let (mut horizontals, mut verticals): (Segments, Segments) = (vec![], vec![]);
    for idx in line.horizontals.iter() {
        horizontals.push(line.segments.get(*idx).unwrap().clone())
    }
    for idx in line.verticals.iter() {
        verticals.push(line.segments.get(*idx).unwrap().clone())
    }

    (horizontals, verticals)
}

fn calculate_intersections(horizontals: Segments, verticals: Segments) -> Intersections {
    let mut intersections: Intersections = vec![];
    for h in horizontals.iter() {
        for v in verticals.iter() {
            let (x1, x2, xv) = (h.p1.x, h.p2.x, v.p1.x);
            let (y1, y2, yh) = (v.p1.y, v.p2.y, h.p1.y);

            if route(x1, x2).contains(&xv) && route(y1, y2).contains(&yh) {
                let h_steps = if x1 > xv { x1 - xv } else { xv - x1 };
                let v_steps = if y1 > yh { y1 - yh } else { yh - y1 };
                let steps = h_steps.abs() + v_steps.abs();

                let point = Point { x: xv, y: yh };
                let cost = h.last_steps + v.last_steps + steps;
                intersections.push(Intersection { point, cost })
            }
        }
    }
    intersections
}

// well, we cannot do range lookups if it is decreasing ...
fn route(a: Coord, b: Coord) -> std::ops::RangeInclusive<Coord> {
    if a > b { b..=a } else { a..=b }
}

fn collect_intersections_from_lines(wire1: Line, wire2: Line) -> Intersections {
    let (wire1_horizontals, wire1_verticals) = get_directed_segments(wire1);
    let (wire2_horizontals, wire2_verticals) = get_directed_segments(wire2);

    let mut intersections: Intersections = vec![];
    let i1 = calculate_intersections(wire1_horizontals, wire2_verticals);
    intersections.extend_from_slice(&i1);
    let i2 = calculate_intersections(wire2_horizontals, wire1_verticals);
    intersections.extend_from_slice(&i2);
    intersections
}

fn collect_distances(intersections: Intersections) -> Distances {
    let mut distances: Vec<Distance> = intersections
        .iter()
        .map(|inter| inter.point.x.abs() + inter.point.y.abs())
        .filter(|v| v.is_positive())
        .collect();
    distances.sort();
    distances
}

fn get_shortest_distance(wire1: Line, wire2: Line) -> Option<Distance> {
    let intersections = collect_intersections_from_lines(wire1, wire2);
    let distances = collect_distances(intersections);
    match distances.first() {
        Some(v) => Some(v.clone()),
        None => None,
    }
}

fn collect_steps(intersections: Intersections) -> Vec<Steps> {
    let mut steps: Vec<Steps> = intersections
        .iter()
        .map(|inter| (inter.point.x.abs() + inter.point.y.abs(), inter.cost) )
        .filter(|(d,_)| d.is_positive())
        .map(|(_,v)| v)
        .collect();
    steps.sort();
    steps
}

fn get_shortest_steps(wire1: Line, wire2: Line) -> Option<Distance> {
    let intersections = collect_intersections_from_lines(wire1, wire2);
    let steps = collect_steps(intersections);
    match steps.first() {
        Some(v) => Some(v.clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tbd() {
        assert!(true);
    }

    #[test]
    fn test_part1_example0() {
        let (ls1, ls2) = (String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4"));
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_distance = get_shortest_distance(wire1, wire2);
        assert_eq!(shortest_distance, Some(6));
    }

    #[test]
    fn test_part2_example0() {
        let (ls1, ls2) = (String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4"));
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_steps = get_shortest_steps(wire1, wire2);
        assert_eq!(shortest_steps, Some(30));
    }

    #[test]
    fn test_part1_example1() {
        let (ls1, ls2) = (
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        );
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_distance = get_shortest_distance(wire1, wire2);
        assert_eq!(shortest_distance, Some(159));
    }

    #[test]
    fn test_part2_example1() {
        let (ls1, ls2) = (
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        );
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_steps = get_shortest_steps(wire1, wire2);
        assert_eq!(shortest_steps, Some(610));
    }

    #[test]
    fn test_part1_example2() {
        let (ls1, ls2) = (
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        );
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_distance = get_shortest_distance(wire1, wire2);
        assert_eq!(shortest_distance, Some(135));
    }

    #[test]
    fn test_part2_example2() {
        let (ls1, ls2) = (
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        );
        let (wire1, wire2) = string2lines(ls1, ls2);
        let shortest_steps = get_shortest_steps(wire1, wire2);
        assert_eq!(shortest_steps, Some(410));
    }
}
