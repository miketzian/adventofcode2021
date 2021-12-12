use std::collections::HashSet;
type CalculationInput = Vec<u8>;
type DayResult = u64;

fn find_neighbors(all: &[Vec<u8>], i: usize, j: usize) -> Vec<(usize, usize, &u8)> {
    let find = |x: usize, y: usize| {
        if let Some(row) = all.get(x) {
            if let Some(value) = row.get(y) {
                return Some((x, y, value));
            }
        }
        None
    };
    let mut bors = Vec::new();

    // usize=0 - 1 will panic
    if i > 0 {
        if let Some(neighbor) = find(i - 1, j) {
            bors.push(neighbor);
        }
    }
    if j > 0 {
        if let Some(neighbor) = find(i, j - 1) {
            bors.push(neighbor);
        }
    }

    if let Some(neighbor) = find(i + 1, j) {
        bors.push(neighbor);
    }
    if let Some(neighbor) = find(i, j + 1) {
        bors.push(neighbor);
    }

    bors
}

pub fn find_low_points(all: &[Vec<u8>]) -> Vec<(usize, usize, u8)> {
    // find low points in the data

    let mut results: Vec<(usize, usize, u8)> = Vec::new();

    for (i, x_row) in all.iter().enumerate() {
        'index: for (j, value) in x_row.iter().enumerate() {
            // all arrays need to be the same size for this loop
            // x must be >0 to use usize

            if find_neighbors(all, i, j)
                .iter()
                .any(|(_, _, neighbor)| **neighbor <= *value)
            {
                continue 'index;
            }
            // if we got here, then i,j is a low point
            println!("found low point with value: {} ({}x{})", *value, i, j);
            // for (x, y, value) in find_neighbors(all, i, j) {
            //     println!("n: {}x{} = {}", x, y, value);
            // }
            results.push((i, j, *value));
        }
    }
    results
}
pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let all: Vec<Vec<u8>> = _input.collect();
    let mut risk: DayResult = 0;

    for (_, _, value) in find_low_points(&all) {
        risk += value as u64 + 1
    }
    risk
}

pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let all: Vec<Vec<u8>> = _input.collect();

    let mut top_three: Vec<usize> = Vec::new();

    for (x, y, value) in find_low_points(&all) {
        // starting from this low point, find the size
        let mut seen = HashSet::new();

        let mut next: Vec<(usize, usize, &u8)> = vec![(x, y, &value)];

        while !next.is_empty() {
            let to_check = next;
            next = Vec::new();

            // these items are by definition low
            // we are looking for items next to these
            // that haven't been seen
            for (cx, cy, _) in to_check {
                for n in find_neighbors(&all, cx, cy) {
                    let (nx, ny, nv) = n;

                    if *nv == 9 {
                        continue;
                    }
                    if seen.contains(&(nx, ny)) {
                        continue;
                    }
                    seen.insert((nx, ny));
                    next.push(n);
                }
            }
        }

        let size = seen.len();

        if top_three.len() < 3 {
            top_three.push(size);
        } else {
            let (i, v) = top_three
                .iter()
                .enumerate()
                .skip(1)
                .fold((0, &top_three[0]), |a, v| if *v.1 < *a.1 { v } else { a });
            if size > *v {
                // this one is bigger than the next smallest
                top_three[i] = size;
            }
        }
    }
    top_three.iter().fold(1, |a, v| a * *v) as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_int_list_from_file("data/day9_test.txt")
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_int_list_from_file("data/day9.txt")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(15, result);

        let result = part1(puzzle_input());
        println!("day 9 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(1134, result);

        let result = part2(puzzle_input());

        println!("day 9 part 2: result={}", result);
    }
}
