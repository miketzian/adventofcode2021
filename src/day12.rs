use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
type CalculationInput = String;
pub struct Point {
    code: String,
    small: bool,
    links: HashSet<String>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}
impl Eq for Point {}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl Point {
    pub fn new(code: String) -> Self {
        let small = if ["start", "end"].contains(&code.as_str()) {
            false
        } else {
            code == code.to_lowercase()
        };
        let links = HashSet::new();
        Point { code, small, links }
    }

    pub fn link(&mut self, other: String) {
        self.links.insert(other);
    }

    pub fn is_start(&self) -> bool {
        self.code == "start".to_string()
    }
}

struct Tracker {
    point_map: HashMap<String, Point>,
}

impl Tracker {
    pub fn new(_input: impl Iterator<Item = String>) -> Self {
        let mut tracker = Tracker {
            point_map: HashMap::new(),
        };
        for line in _input {
            // from-to
            let (from_s, to_s) = line
                .trim()
                .split('-')
                .collect_tuple()
                .expect("this is a tuple");

            tracker.track(from_s, to_s);
        }
        tracker
    }
    pub fn track(&mut self, from: &str, to: &str) {
        let (from_s, to_s) = (from.to_string(), to.to_string());

        // so much clone-ing!
        self.point_map
            .entry(from_s.clone())
            .or_insert(Point::new(from_s.clone()))
            .link(to_s.clone());

        self.point_map
            .entry(to_s.clone())
            .or_insert(Point::new(to_s.clone()))
            .link(from_s.clone());
    }

    pub fn start(&self) -> &Point {
        self.point_map.get("start").unwrap()
    }

    pub fn iter_next(&self, code: String) -> impl Iterator<Item = &Point> {
        let this_point = self.point_map.get(&code.clone()).expect("to be present");

        this_point.links.iter().cloned().filter_map(|code| {
            let next_point = self.point_map.get(&code).unwrap();
            if !next_point.is_start() {
                Some(next_point)
            } else {
                None
            }
        })
    }

    pub fn traverse_part1(&self, mut path: Vec<String>, next: &Point) -> Vec<Vec<String>> {
        path.push(next.code.clone());

        let mut r = vec![];
        if next.code.eq("end") {
            r.push(path);
            return r;
        }

        for mut option in self.iter_next(next.code.clone()).filter_map(|point| {
            let path_copied = path.clone();

            if point.small && path.contains(&point.code) {
                None
            } else {
                Some(self.traverse_part1(path_copied, point))
            }
        }) {
            r.append(&mut option);
        }

        return r;
    }

    pub fn traverse_part2(&self, mut path: Vec<String>, next: &Point) -> Vec<Vec<String>> {
        path.push(next.code.clone());

        let mut r = vec![];
        if next.code.eq("end") {
            r.push(path);
            return r;
        }

        for mut option in self.iter_next(next.code.clone()).filter_map(|point| {
            let path_copied = path.clone();

            if point.small && path.contains(&point.code) {
                // if no other point has two, then we can do this one
                let mut small_map = HashMap::new();
                for x in &path {
                    if Point::new(x.clone()).small {
                        *small_map.entry(x.clone()).or_insert(0) += 1;
                    }
                }
                // if all other points have one or none, then add
                if small_map.iter().all(|(_, v)| *v == 1) {
                    Some(self.traverse_part2(path_copied, point))
                } else {
                    // otherwise, we return none
                    None
                }
            } else {
                Some(self.traverse_part2(path_copied, point))
            }
        }) {
            r.append(&mut option);
        }
        r
    }
}

pub fn part1(_input: impl Iterator<Item = String>) -> u64 {
    // when we process a point, then make it here
    let tracker = Tracker::new(_input);

    let start = tracker.start();
    let mut paths = 0;
    for path in tracker.traverse_part1(vec![], start) {
        print!("{}", path[0]);
        for p in path.iter().skip(1) {
            print!(",{}", p);
        }
        println!();
        paths += 1;
    }
    paths
}

pub fn part2(_input: impl Iterator<Item = String>) -> u64 {
    let tracker = Tracker::new(_input);

    let start = tracker.start();
    let mut paths = 0;
    for _path in tracker.traverse_part2(vec![], start) {
        // print!("{}", _path[0]);
        // for p in _path.iter().skip(1) {
        //     print!(",{}", p);
        // }
        // println!();
        paths += 1;
    }
    paths
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data(case: u8) -> impl Iterator<Item = CalculationInput> {
        let source = format!("data/day{}_test_{}.txt", 12, case);
        super::super::util::read_file(source.as_str())
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_data(12, false)
    }

    #[test]
    fn test_part1() {
        let mut result = part1(test_data(1));
        assert_eq!(10, result);

        result = part1(test_data(2));
        assert_eq!(19, result);

        result = part1(test_data(3));
        assert_eq!(226, result);

        // 5756
        let result = part1(puzzle_input());
        println!("day 10 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let mut result = part2(test_data(1));
        assert_eq!(36, result);

        result = part2(test_data(2));
        assert_eq!(103, result);

        result = part2(test_data(3));
        assert_eq!(3509, result);
        let result = part2(puzzle_input());

        // 144603
        println!("day 10 part 2: result={}", result);
    }
}
