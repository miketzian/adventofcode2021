use std::collections::HashMap;
type ParseResult = Result<CalculationInput, String>;
type CalculationInput = HashMap<u64, u64>;
type DayResult = (u64, u64);

pub fn parse_line(input: String) -> ParseResult {
    Result::Ok(
        input
            .trim()
            .split(',')
            .map(|v| v.parse::<u64>().expect("should be int"))
            .fold(HashMap::new(), |mut acc, i| {
                if let Some(v) = acc.remove(&i) {
                    acc.insert(i, v + 1);
                } else {
                    acc.insert(i, 1);
                }
                acc
            }),
    )
}

pub fn calculate_fuel_constant(candidate: &u64, input: &CalculationInput) -> u64 {
    input.iter().fold(0, |acc: u64, (k, v)| {
        let fuel = if *candidate > *k {
            (*candidate - *k) * v
        } else {
            (*k - *candidate) * v
        };
        acc + fuel
    })
}

pub fn calculate_fuel_increasing(candidate: &u64, input: &CalculationInput) -> u64 {
    input.iter().fold(0, |acc: u64, (k, v)| {
        let mut steps = if *candidate > *k {
            *candidate - *k
        } else {
            *k - *candidate
        };

        let mut fuel = 0;
        let mut cost = 1;
        while steps > 0 {
            fuel += cost * *v;
            steps -= 1;
            cost += 1;
        }
        acc + fuel
    })
}

pub fn part1(input: CalculationInput) -> DayResult {
    calculate(input, calculate_fuel_constant)
}

pub fn part2(input: CalculationInput) -> DayResult {
    calculate(input, calculate_fuel_increasing)
}

pub fn calculate(
    input: CalculationInput,
    calculate_fuel: impl Fn(&u64, &CalculationInput) -> u64,
) -> DayResult {
    let (sum, entries) = input.iter().fold((0, 0), |acc: (u64, u64), (k, v)| {
        let (mut sum, mut entries) = acc;
        sum += *k as u64 * *v;
        entries += *v;
        (sum, entries)
    });
    // println!("sum={}, entries={}", sum, entries);

    let mut candidate = (sum / entries) as u64;
    // start from the candidate
    let mut candidate_score = calculate_fuel(&candidate, &input);

    // println!("candidate={}, score={}", candidate, candidate_score);

    let mut found_alternate = true;

    let mut calculated = HashMap::new();

    while found_alternate {
        found_alternate = false;

        for alternate in (if candidate > 3 { candidate - 3 } else { 0 })..(candidate + 3) {
            if calculated.get(&alternate).is_none() {
                // println!("checking {}", alternate);
                // we haven't already checked this number
                let alternate_score = calculate_fuel(&alternate, &input);
                calculated.insert(alternate, alternate_score);
                if alternate_score < candidate_score {
                    candidate = alternate;
                    candidate_score = alternate_score;
                    // println!("found: candidate={}, score={}", candidate, candidate_score);
                    found_alternate = true;
                }
            }
        }
    }

    (candidate, candidate_score)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> CalculationInput {
        super::super::util::parse_file("data/day7_test.txt", parse_line)
            .next()
            .expect("there should be one line")
    }

    fn puzzle_input() -> CalculationInput {
        super::super::util::parse_file("data/day7.txt", parse_line)
            .next()
            .expect("there should be one line")
    }

    #[test]
    fn test_part1() {
        let (result, fuel) = part1(test_data());

        assert_eq!(2, result);
        assert_eq!(37, fuel);

        let (result, fuel) = part1(puzzle_input());

        println!("day 7 part 1: result={}, fuel={}", result, fuel);
    }

    #[test]
    fn test_part2() {
        let (result, fuel) = part2(test_data());

        assert_eq!(5, result);
        assert_eq!(168, fuel);

        let (result, fuel) = part2(puzzle_input());

        println!("day 7 part 2: result={}, fuel={}", result, fuel);
    }
}
