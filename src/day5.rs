type CalculationInput = (i32, i32, i32, i32);
type DayResult = i32;
type ParseResult = Result<CalculationInput, String>;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Parse a day 5 line into fx,fy,tx,ty
/// ```
/// use aoc2021::day5;
///
/// match day5::parse_line("0,9 -> 5,9".to_string()) {
///     Ok((fx,fy,tx,ty)) => {
///         assert_eq!(0, fx);
///         assert_eq!(9, fy);
///         assert_eq!(5, tx);
///         assert_eq!(9, ty);
///     },
///     Err(_) => unreachable!(),
/// };
/// match day5::parse_line("nothing".to_string()) {
///     Err(msg) => assert_eq!("error parsing: nothing", msg),
///     Ok(_) => unreachable!(),
/// };
/// ```
pub fn parse_line(input: String) -> ParseResult {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+) .. (\d+),(\d+)$").unwrap();
    }

    if let Some(cap) = RE.captures(input.as_str()) {
        // capture 0 is the full line
        assert_eq!(cap.len(), 5);
        // capture 1-4 are the matches
        Ok((
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        ))
    } else {
        Err(format!("error parsing: {}", input))
    }
}

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    calculate(_input, false)
}

pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    calculate(_input, true)
}

pub fn calculate(_input: impl Iterator<Item = CalculationInput>, count_diag: bool) -> DayResult {
    let mut counts: HashMap<(i32, i32), bool> = HashMap::new();

    // this cannot be a fn, as it needs access to the local scope.
    // closures have access to the local scope.

    // not sure why the function itself is mutable, but it doesn't
    // run without it being so.
    let mut add_counts = |x: i32, y: i32| {
        let key = (x, y);
        match counts.get(&key) {
            Some(v) if !*v => {
                counts.insert(key, true);
            }
            Some(_) => (),
            None => {
                counts.insert(key, false);
            }
        }
    };

    for (mut fx, mut fy, tx, ty) in _input {
        // calculate the detla between f->t
        let xd: i32 = match fx {
            x if x > tx => -1,
            x if x < tx => 1,
            _ => 0,
        };

        let yd = match fy {
            y if y > ty => -1,
            y if y < ty => 1,
            _ => 0,
        };

        if !(count_diag || xd == 0 || yd == 0) {
            // first pass, we only care about horizontal/vertical
            continue;
        }

        // add the initial point
        add_counts(fx, fy);
        // then iterate until we get to the final point
        while !(fx == tx && fy == ty) {
            fx += xd;
            fy += yd;
            add_counts(fx, fy);
        }
    }
    // how many true values (more than one match) in the map
    counts.values().fold(0, |a, v| a + if *v { 1 } else { 0 })
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day5_test.txt", parse_line)
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day5.txt", parse_line)
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(5, result);

        let result = part1(puzzle_input());

        println!("day 5 part 1: {}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(12, result);

        let result = part2(puzzle_input());

        println!("day 5 part 2: {}", result);
    }

    #[test]
    fn test_cov() {
        match parse_line("nothing".to_string()) {
            Err(msg) => assert_eq!("error parsing: nothing", msg),
            Ok(_) => unreachable!(),
        };
    }
}
