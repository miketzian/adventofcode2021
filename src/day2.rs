/// This is my first rust doctest
/// ```
/// use aoc2021::day2::part1;
///
/// let case = vec![("down", 3), ("forward", 6), ("up", 1)];
///
/// let iter = case.iter().map(|(s, i)| (s.to_string(), *i));
///
/// let (depth, horiz, distance) = part1(iter);
///
/// assert_eq!(2, depth);
/// assert_eq!(6, horiz);
/// assert_eq!(12, distance);
/// ```
pub fn part1(instructions: impl Iterator<Item = (String, i32)>) -> (i32, i32, i32) {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for (direction, value) in instructions {
        match direction.as_str() {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => horiz += value,
            _ => unreachable!(),
        }
    }
    (depth, horiz, depth * horiz)
}

pub fn part2(instructions: impl Iterator<Item = (String, i32)>) -> (i32, i32, i32, i32) {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;
    let mut aim: i32 = 0;

    for (direction, value) in instructions {
        match direction.as_str() {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                horiz += value;
                depth += aim * value;
            }
            _ => unreachable!(),
        }
    }
    (depth, horiz, depth * horiz, aim)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = (String, i32)> {
        super::super::util::read_string_int_from_file("data/day2_test.txt".to_string())
    }

    fn puzzle_input() -> impl Iterator<Item = (String, i32)> {
        super::super::util::read_string_int_from_file("data/day2.txt".to_string())
    }

    #[test]
    fn test_part1() {
        let (depth, horiz, distance) = part1(test_data());

        assert_eq!(10, depth);
        assert_eq!(15, horiz);
        assert_eq!(150, distance);

        let (depth, horiz, distance) = part1(puzzle_input());

        println!(
            "day2 part 1: depth={}, horizontal={}, distance={}",
            depth, horiz, distance
        );
    }

    #[test]
    fn test_part2() {
        let (depth, horiz, distance, aim) = part2(test_data());

        assert_eq!(60, depth);
        assert_eq!(15, horiz);
        assert_eq!(900, distance);
        assert_eq!(10, aim);

        let (depth, horiz, distance, aim) = part2(puzzle_input());

        println!(
            "day2 part 2: depth={}, horizontal={}, distance={}, aim={}",
            depth, horiz, distance, aim
        );
    }
}
