use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type CalculationInput = String;
type DayResult = usize;

type Paper = Vec<Vec<bool>>;

pub struct Command {
    dir: char,
    value: usize,
}

impl Command {
    pub fn new(dir: char, value: usize) -> Self {
        assert!(['x', 'y'].contains(&dir));
        Command { dir, value }
    }

    pub fn run(&self, paper: Paper) -> Paper {
        // run this command on the paper, returning the result.
        match self.dir {
            'x' => fold_vertical_line(self.value, paper),
            'y' => fold_horizontal_line(self.value, paper),
            _ => unreachable!(),
        }
    }
}

pub fn parse_command(input: String) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^fold along (x|y)=([0-9]+)$").unwrap();
    }
    // fold along y=7
    // fold along x=5
    if let Some(cap) = RE.captures(input.as_str()) {
        assert_eq!(cap.len(), 3);
        Command::new(
            cap.get(1).unwrap().as_str().chars().next().unwrap(),
            cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        )
    } else {
        unreachable!();
    }
}

pub fn load_data(
    mut _input: impl Iterator<Item = CalculationInput>,
) -> (Vec<Vec<bool>>, Vec<Command>) {
    let mut hits = Vec::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for hit in _input.by_ref().take_while(|v| !v.is_empty()) {
        let tup: (usize, usize) = hit
            .trim()
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        hits.push(tup);
        if tup.0 > max_x {
            max_x = tup.0;
        }
        if tup.1 > max_y {
            max_y = tup.1;
        }
    }

    let mut paper = vec![vec![false; max_x + 1]; max_y + 1];
    for (x, y) in hits {
        paper[y][x] = true;
    }

    let commands: Vec<Command> = _input.map(parse_command).collect();
    (paper, commands)
}

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let (mut paper, commands): (Vec<Vec<bool>>, Vec<Command>) = load_data(_input);

    for command in commands.iter().take(1) {
        println!("fold -> {} -> {}", command.dir, command.value);
        paper = command.run(paper);
        // for line in paper.clone() {
        //     for v in line {
        //         if v {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
    }

    paper
        .iter()
        .map(|r: &Vec<bool>| r.iter().filter(|v| **v).count())
        .sum()
}

pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let (mut paper, commands): (Vec<Vec<bool>>, Vec<Command>) = load_data(_input);

    for command in commands {
        println!("fold -> {} -> {}", command.dir, command.value);
        paper = command.run(paper);
    }

    // this will print out the result
    for line in paper.clone() {
        for v in line {
            if v {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    paper
        .iter()
        .map(|r: &Vec<bool>| r.iter().filter(|v| **v).count())
        .sum()
}

// in this case, we fold within a line
pub fn fold_vertical_line(x: usize, paper: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    paper
        .iter()
        .map(|row| {
            assert!(row.len() > x);
            (0..x).map(|ix| row[ix] || row[(x * 2) - ix]).collect()
        })
        .collect()
}

// in this case, we fold rows up
pub fn fold_horizontal_line(y: usize, paper: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    paper
        .iter()
        .take(y)
        .enumerate()
        .map(|(row_ix, data)| {
            // rix=0, y=3, o=6
            let other_row = y * 2 - row_ix;
            data.iter()
                .enumerate()
                .map(|(col_ix, value)| paper[other_row][col_ix] || *value)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_file("data/day13_test.txt")
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_file("data/day13.txt")
    }

    #[test]
    fn test_fold_vertical() {
        let mut data: Vec<Vec<bool>> = vec![
            vec![true, false, true, false, false, true, false],
            vec![false, false, false, false, false, false, true],
        ];
        data = fold_vertical_line(3, data);
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].len(), 3);
        assert!(data[0][0]);
        assert!(data[0][1]);
        assert!(data[0][2]);

        assert_eq!(data[1].len(), 3);
        assert!(data[1][0]);
        assert!(!data[1][1]);
        assert!(!data[1][2]);
    }

    #[test]
    fn test_fold_horizontal() {
        let mut data: Vec<Vec<bool>> = vec![
            vec![true, false, true, false, false, true, false],
            vec![true, false, true, false, false, true, false],
            vec![false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false],
            vec![false, true, false, true, true, false, true],
        ];
        data = fold_horizontal_line(2, data);
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].len(), 7);
        for i in 0..7 {
            assert!(data[0][i]);
        }
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(17, result);

        // ! not 95
        let result = part1(puzzle_input());
        println!("day 13 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(16, result);

        let result = part2(puzzle_input());

        println!("day 13 part 2: result={}", result);
    }
}
