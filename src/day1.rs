pub fn larger_measurements(input: impl Iterator<Item = i32>) -> u32 {
    let mut prev: Option<i32> = None;
    let mut count: u32 = 0;
    for value in input {
        if let Some(previous) = prev {
            if value > previous {
                count += 1;
            }
        }
        prev = Some(value);
    }
    count
}

pub fn larger_triples(input: impl Iterator<Item = i32>) -> u32 {
    let mut prev: Option<i32> = None;
    let mut count: u32 = 0;
    for value in super::util::TripleIter::new(Box::new(input)) {
        if let Some(previous) = prev {
            if value > previous {
                count += 1;
            }
        }
        prev = Some(value);
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    // fn test_data() -> Vec<i32> {
    //     let data: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     data
    // }

    fn test_data() -> impl Iterator<Item = i32> {
        super::super::util::read_ints_from_file("data/day1_test.txt".to_string())
    }

    fn puzzle_input() -> impl Iterator<Item = i32> {
        super::super::util::read_ints_from_file("data/day1.txt".to_string())
    }

    #[test]
    fn test_larger_measurements() {
        // i would ideally define teh test data above, however if i do that I get an Iterator<Item=&i32>
        // but the file-based method returns me Iterator<Item=i32> and I can't seem to reconcile those
        // without turning the file source into a collected Vec

        let result = larger_measurements(test_data());

        assert_eq!(result, 7);

        let result = larger_measurements(puzzle_input());

        println!("day 1 part 1 result: {}", result)
    }

    #[test]
    fn test_larger_triples() {
        let result = larger_triples(test_data());

        assert_eq!(result, 5);

        let result = larger_triples(puzzle_input());

        println!("day 1 part 2 result: {}", result)
    }
}
