use itertools::Itertools;
use std::collections::HashMap;
type ParseResult = Result<CalculationInput, String>;
type CalculationInput = (Vec<String>, Vec<String>);
type DayResult = u64;

pub fn parse_line(input: String) -> ParseResult {
    let mut iter = input
        .trim()
        .split(" | ")
        .map(|v: &str| v.trim().split(' ').map(|s| s.to_string()).collect());

    let r = (
        iter.next().expect("signal patterns"),
        iter.next().expect("output value"),
    );

    if iter.next().is_some() {
        Err("more records in the iterator than we expected".to_string())
    } else {
        Result::Ok(r)
    }
}

pub fn part1(input: impl Iterator<Item = CalculationInput>) -> DayResult {
    // part one is a simple filter for those which have detectable digits

    let mut count: u64 = 0;

    for (_, output_values) in input {
        // use the signal pattern to determine how the map work

        for pattern in output_values {
            match pattern.len() {
                2 => {
                    // cf = 1
                    count += 1;
                }
                3 => {
                    // acf = 7
                    count += 1;
                }
                4 => {
                    // bcdf = 4
                    count += 1;
                }
                7 => {
                    // abcdefg = 8
                    count += 1;
                }
                _ => {} // ignoring these
            }
        }
        // then map the output values to the result
    }
    count
}

pub fn calculate_4hbq(input: CalculationInput) -> DayResult {
    // I got stuck on this one - a brute force solution from reddit converted to rust

    let all = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut values = HashMap::new();

    // 1 - 2 - __c__f_
    values.insert("cf", 1);
    // 7 - 3 - a_c__f_
    values.insert("acf", 7);
    // 4 - 4 - _bcd_f_
    values.insert("bcdf", 4);
    // 8 - 7 - abcdefg
    values.insert("abcdefg", 8);

    // 5 - 5 - ab_d_fg
    values.insert("abdfg", 5);
    // 2 - 5 - a_cde_g
    values.insert("acdeg", 2);
    // 3 - 5 - a_cd_fg
    values.insert("acdfg", 3);

    // 0 - 6 - abc_efg
    values.insert("abcefg", 0);
    // 6 - 6 - ab_defg
    values.insert("abdefg", 6);
    // 9 - 6 - abcd_fg
    values.insert("abcdfg", 9);

    assert_eq!(10, values.len());

    let (signal_patterns, output_values) = input;

    let option = all
        .iter()
        .permutations(all.len())
        .find(|option| {
            // do all the values work with the wiring in this position ?
            let mut all_ok = true;

            for value in &signal_patterns {
                let mut mapped: Vec<char> = value
                    .chars()
                    .map(|c| {
                        let ix = option.iter().position(|o| **o == c).unwrap();
                        *all.get(ix).unwrap()
                    })
                    .collect();

                mapped.sort_unstable();
                let result = mapped.iter().fold(String::new(), |mut a: String, b| {
                    a.push(*b);
                    a
                });

                if !values.contains_key(result.as_str()) {
                    all_ok = false;
                    break;
                }
            }
            all_ok
        })
        .expect("there should be one permutation that works");

    output_values
        .iter()
        .map(|ov| {
            let mut mapped: Vec<char> = ov
                .chars()
                .map(|c| {
                    let ix = option.iter().position(|o| **o == c).unwrap();
                    *all.get(ix).unwrap()
                })
                .collect();

            mapped.sort_unstable();
            let result = mapped.iter().fold(String::new(), |mut a: String, b| {
                a.push(*b);
                a
            });
            values
                .get(result.as_str())
                .expect("there should be a mapping")
        })
        .fold(0, |a: u64, v| (a * 10) + *v)
}

pub fn part2(input: impl Iterator<Item = CalculationInput>) -> DayResult {
    // need a process of elimination to determine which 4 digits this is.
    input.map(calculate_4hbq).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day8_test.txt", parse_line)
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day8.txt", parse_line)
    }

    #[test]
    fn test_single_line() {
        let mut line = String::new();
        line += "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        line += " | cdfeb fcadb cdfeb cdbaf";

        let input = parse_line(line).expect("good line");

        let result = calculate_4hbq(input);

        assert_eq!(result, 5353);
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(26, result);

        let result = part1(puzzle_input());

        println!("day 8 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(61229, result);

        let result = part2(puzzle_input());

        println!("day 8 part 2: result={}", result);
    }
}
