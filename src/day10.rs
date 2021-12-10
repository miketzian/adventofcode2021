use std::collections::HashMap;
type CalculationInput = String;
type DayResult = u64;

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let mut parser = Vec::new();
    let mut invalid = HashMap::new();

    let open_close: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let close_open: HashMap<char, char> = open_close.iter().map(|(k, v)| (*v, *k)).collect();

    for line in _input {
        parser.clear();

        'parsing: for next_char in line.chars() {
            match next_char {
                c if open_close.contains_key(&c) => parser.push(c),
                _ => {
                    // must be a close char
                    if let Some(last_char) = parser.pop() {
                        // what is the open for this char
                        let open_char = *close_open
                            .get(&next_char)
                            .expect("if we are not an open, then we must be a close");

                        if open_char == last_char {
                            // next_char closes last_char, so this is good
                            continue 'parsing;
                        }
                        // this string is bad, fall through to next line
                    }
                    // else we had no chars, so it's bad

                    if let Some(v) = invalid.remove(&next_char) {
                        invalid.insert(next_char, v + 1);
                    } else {
                        invalid.insert(next_char, 1);
                    }
                }
            }
        }
        // all ok! we ignore this line.
    }

    invalid.iter().fold(0, |acc, (k, v)| {
        acc + (match *k {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        } * *v)
    })
}

/// in part two, we want to match on the incomplete lines
pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let mut parser = Vec::new();

    // immutable hash map
    let open_close: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let close_open: HashMap<char, char> = open_close.iter().map(|(k, v)| (*v, *k)).collect();

    let mut score: Vec<u64> = Vec::new();

    'outer: for line in _input {
        parser.clear();
        'parsing: for next_char in line.chars() {
            match next_char {
                c if open_close.contains_key(&c) => parser.push(c),
                _ => {
                    // must be a close char
                    if let Some(last_char) = parser.pop() {
                        // what is the open for this char
                        let open_char = *close_open
                            .get(&next_char)
                            .expect("if we are not an open, then we must be a close");

                        if open_char == last_char {
                            // next_char closes last_char, so this is good
                            continue 'parsing;
                        }
                        // this string is bad, fall through to next line
                    }
                    // else we had no chars, so it's bad
                    continue 'outer;
                }
            }
        }
        // all ok, but probably we need to close this line

        // println!("incomplete: {}", line);
        // println!(
        //     "remaining: {}",
        //     parser
        //         .iter()
        //         .fold(String::new(), |mut a: String, b: &char| {
        //             a.push(*b);
        //             a
        //         })
        // );

        // we need to fold in reverse order as we are closing the items
        let this_score = parser.iter().rev().fold(0, |acc: u64, c| {
            (acc * 5)
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                }
        });
        // println!("score: {}", this_score);
        score.push(this_score);
    }
    // we want the middle score by value
    score.sort_unstable();
    score[((score.len() - 2) / 2) + 1]
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_data(10, true)
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_data(10, false)
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(26397, result);

        let result = part1(puzzle_input());
        println!("day 10 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(288957, result);

        let result = part2(puzzle_input());

        println!("day 10 part 2: result={}", result);
    }
}
