// use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cell::Cell;
use std::collections::HashMap;
use typed_arena::Arena;

type CalculationInput = Vec<usize>;
type DayResult = u32;

pub struct Node<'a> {
    key: (usize, usize),
    risk: usize,
    edges: Vec<Cell<&'a Node<'a>>>,
}

pub fn file_to_map<'a>(
    input: impl Iterator<Item = CalculationInput>,
) -> (Arena<Node<'a>>, HashMap<(usize, usize), &'a mut Node<'a>>) {
    let nodes: Arena<Node<'a>> = Arena::new();
    let mut data = HashMap::new();
    {
        for (x, line) in input.enumerate() {
            for (y, value) in line.iter().enumerate() {
                let node = nodes.alloc(Node {
                    key: (x, y),
                    risk: *value,
                    edges: vec![],
                });

                data.insert((x, y), node);
            }
        }

        // let d2: HashMap<(usize, usize), &Node<'static>> = data.iter().map(|(k, v)| (*k, v)).collect();

        for node in data.values() {
            let (i, j) = node.key;
            let mut edges = *node.edges;
            if i > 0 {
                if let Some(edge) = data.get(&(i - 1, j)) {
                    edges.push(Cell::new(edge));
                }
            }
            if j > 0 {
                if let Some(edge) = &data.get(&(i, j - 1)) {
                    edges.push(Cell::new(edge));
                }
            }

            if let Some(edge) = &data.get(&(i + 1, j)) {
                edges.push(Cell::new(edge));
            }
            if let Some(edge) = &data.get(&(i, j + 1)) {
                edges.push(Cell::new(edge));
            }
            // ((i, j), node)
        }
    }

    (nodes, data)
}

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let (arena, data) = file_to_map(_input);

    let mut q = PriorityQueue::new();
    let mut dist = HashMap::new();
    dist.insert((0, 0), 0);
    q.push((0, 0), 0);

    while let Some(((x, y), _)) = q.pop() {}

    // run(_input, 10)
    0
}

pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    // run(_input, 40)
    0
}

fn parse_line(input: String) -> Result<CalculationInput, String> {
    Result::Ok(
        input
            .chars()
            .map(|v| v.to_string().parse::<usize>().expect("all should parse"))
            .collect(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day15_test.txt", parse_line)
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::parse_file("data/day15.txt", parse_line)
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(40, result);

        // ! not 95
        let result = part1(puzzle_input());
        println!("day 15 part 1: result={}", result);

        assert_eq!(456, result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());
        assert_eq!(315, result);

        let result = part2(puzzle_input());

        println!("day 15 part 2: result={}", result);
    }
}
