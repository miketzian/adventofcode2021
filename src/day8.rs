use itertools::Itertools;
use std::collections::HashMap;
type ParseResult = Result<CalculationInput, String>;
type CalculationInput = (Vec<String>, Vec<String>);
type DayResult = u64;

pub fn parse_line(input: String) -> ParseResult {
    let mut iter = input.trim().split(" | ").map(|v: &str| {
        v.trim()
            .split(' ')
            .map(|s| {
                let mut sc: Vec<char> = s.chars().collect_vec();
                sc.sort_unstable();
                sc.iter().fold(String::new(), |mut a, b| {
                    a.push(*b);
                    a
                })
            })
            .collect()
    });

    let mut signal: Vec<String> = iter.next().expect("signal patterns");
    let output: Vec<String> = iter.next().expect("output value");

    if iter.next().is_some() {
        Err("more records in the iterator than we expected".to_string())
    } else {
        // smallest to largest by length
        signal.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        Result::Ok((signal, output))
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

/// return true if all char in str is in str2
fn all_in(str: &str, str2: &str) -> bool {
    str.chars().all(|c| str2.contains(c))
}

pub fn calculate(input: CalculationInput) -> DayResult {
    // signal_patterns are sorted in length order ..
    let (signal_patterns, output_values) = input;

    let mut entries: HashMap<String, u64> = HashMap::new();

    // so the 1st will be 1 with len=2
    let one = &signal_patterns[0];
    entries.insert(signal_patterns[0].to_string(), 1);

    // 2nd will be 7 with len=3
    // let seven = &signal_patterns[1];
    entries.insert(signal_patterns[1].to_string(), 7);

    // 3rd 4 with len=4
    let four = &signal_patterns[2];
    entries.insert(signal_patterns[2].to_string(), 4);

    // and last 8 with len=7
    entries.insert(signal_patterns[9].to_string(), 8);

    // six
    // nine
    // zero

    let mut six: &String = &"".to_string();

    // 6,7,8
    // for i in 6..9 {
    //     let v = &signal_patterns[i];
    for v in signal_patterns.iter().take(9).skip(6) {
        if all_in(four, v) {
            // this is 9
            entries.insert(v.to_string(), 9);
        } else if all_in(one, v) {
            // this is zero
            entries.insert(v.to_string(), 0);
        } else {
            // six
            entries.insert(v.to_string(), 6);
            six = v;
        }
    }

    // two
    // three
    // five

    // 3,4,5
    // for i in 3..6 {
    //     let v = &signal_patterns[i];
    for v in signal_patterns.iter().take(6).skip(3) {
        // 3,4,5
        if all_in(one, v) {
            // three
            entries.insert(v.to_string(), 3);
        } else if all_in(v, six) {
            // five
            entries.insert(v.to_string(), 5);
        } else {
            entries.insert(v.to_string(), 2);
        }
    }
    // now we know what all the values are, we can map to the result
    output_values
        .iter()
        .map(|v| entries.get(v).unwrap())
        .fold(0, |acc, vv| (acc * 10) + *vv)
}

pub fn part2(input: impl Iterator<Item = CalculationInput>) -> DayResult {
    input.map(calculate).sum()
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

        let result = calculate(input);

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
