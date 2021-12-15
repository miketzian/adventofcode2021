use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type CalculationInput = String;
type DayResult = u128;

pub fn run(mut _input: impl Iterator<Item = CalculationInput>, steps: u8) -> DayResult {
    let start = _input
        .by_ref()
        .take_while(|v| !v.is_empty())
        .next()
        .expect("present");

    let mut counts: HashMap<(char, char), DayResult> =
        (0..start.len() - 1).fold(HashMap::new(), |mut acc, x| {
            let value: (char, char) = start.chars().skip(x).take(2).collect_tuple().unwrap();
            if let Some(y) = acc.remove(&value) {
                acc.insert(value, y + 1);
            } else {
                acc.insert(value, 1);
            }
            acc
        });

    _input.next();

    let mapping: HashMap<(char, char), char> = _input
        .map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^([A-Z][A-Z]) .. ([A-Z])$").unwrap();
            }
            if let Some(cap) = RE.captures(line.as_str()) {
                (
                    cap.get(1)
                        .unwrap()
                        .as_str()
                        .chars()
                        .collect_tuple()
                        .unwrap(),
                    cap.get(2).unwrap().as_str().chars().next().unwrap(),
                )
            } else {
                println!("unreachable [{}]", line);
                unreachable!();
            }
        })
        .collect();

    for _step in 0..steps {
        let mut counts_now = counts.clone();
        for (frag, v) in counts.iter() {
            if let Some(new_char) = mapping.get(frag) {
                *counts_now.entry((frag.0, frag.1)).or_insert(0) -= *v;
                *counts_now.entry((frag.0, *new_char)).or_insert(0) += *v;
                *counts_now.entry((*new_char, frag.1)).or_insert(0) += *v;
            }
        }
        counts = counts_now;
    }

    let mut result = HashMap::new();
    for (k, v) in counts.iter() {
        // println!("{},{} = {}", k.0, k.1, v);
        *result.entry(k.0).or_insert(0 as f64) += (*v / 2) as f64;
        *result.entry(k.1).or_insert(0 as f64) += (*v / 2) as f64;
    }

    let mut count_iter = result.iter();
    let first = count_iter.next().unwrap();

    let mut min = *first.1;
    let mut max = *first.1;

    for (_char, v) in count_iter {
        println!("{}={}", _char, v);
        if *v < min {
            min = *v;
        }
        if *v > max {
            max = *v;
        }
    }
    println!("max={}, min={}", max, min);
    (max.ceil() - min.ceil()) as u128
}

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    run(_input, 10)
}

pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    run(_input, 40)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_file("data/day14_test.txt")
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_file("data/day14.txt")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(1588, result);

        // ! not 95
        let result = part1(puzzle_input());
        println!("day 14 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());
        assert_eq!(2188189693529, result);

        let result = part2(puzzle_input());

        println!("day 14 part 2: result={}", result);
    }
}
