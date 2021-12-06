use std::collections::HashMap;
type ParseResult = Result<CalculationInput, String>;
type CalculationInput = HashMap<u8, u64>;
type DayResult = u64;

pub fn parse_line(input: String) -> ParseResult {
    Result::Ok(
        input
            .trim()
            .split(',')
            .map(|v| v.parse::<u8>().expect("should be int"))
            .fold(HashMap::new(), |mut acc, i| {
                match acc.get(&i) {
                    Some(v) => {
                        let inc = v + 1;
                        acc.insert(i, inc)
                    }
                    None => acc.insert(i, 1),
                };
                acc
            }),
    )
}

fn process(input: CalculationInput, days: u32) -> DayResult {
    let mut fish = input;

    for _day in 0..days {
        let new_fish = if let Some(vv) = fish.remove(&0) {
            // we have a value
            vv
        } else {
            0
        };
        for (age, age_minus_one) in (1..9).map(|v| (v as u8, (v - 1) as u8)) {
            if let Some(fish_count) = fish.remove(&age) {
                fish.insert(age_minus_one, fish_count);
            } else {
                // remove if it's there.
                fish.remove(&age_minus_one);
            }
        }
        if new_fish > 0 {
            // 8 should not have a value, since if it did it would have been moved
            // in the previous step
            {
                fish.insert(8, new_fish);
            }
            // 6 may already have a value, so check first
            // we remove it so that we don't have to later assign
            // it to a variable
            if let Some(v) = fish.remove(&6) {
                fish.insert(6, v + new_fish);
            } else {
                fish.insert(6, new_fish);
            }
        }
        // let day_sum: u64 = today.values().fold(0, |acc, v| acc + *v as u64);
        // println!("day {} -> {} fish", _day, day_sum);
    }
    fish.values().sum()
}

pub fn part1(input: CalculationInput) -> DayResult {
    process(input, 80)
}

pub fn part2(input: CalculationInput) -> DayResult {
    process(input, 256)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> CalculationInput {
        super::super::util::parse_file("data/day6_test.txt", parse_line)
            .next()
            .expect("there should be one line")
    }

    fn puzzle_input() -> CalculationInput {
        super::super::util::parse_file("data/day6.txt", parse_line)
            .next()
            .expect("there should be one line")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(5934, result);

        let result = part1(puzzle_input());

        println!("day 6 part 1: {}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(26984457539, result);

        let result = part2(puzzle_input());

        println!("day 6 part 2: {}", result);
    }
}
