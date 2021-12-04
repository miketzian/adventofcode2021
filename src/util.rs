use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;

/// ```should_panic
/// use aoc2021::util;
/// let _ = util::read_file("/not/exist.txt".to_string());
///```
pub fn read_file(file_path: String) -> io::BufReader<fs::File> {
    let path = path::Path::new(&file_path);

    if let Ok(file) = fs::File::open(path) {
        io::BufReader::new(file)
    } else {
        // at least, within this repo
        unreachable!();
    }
}

pub fn read_strings_from_file(file_path: String) -> impl Iterator<Item = String> {
    read_file(file_path).lines().filter_map(Result::ok)
}

pub fn read_ints_from_file(file_path: String) -> impl Iterator<Item = i32> {
    read_file(file_path)
        .lines()
        .filter_map(Result::ok)
        .map(|v| v.parse::<i32>())
        .filter_map(Result::ok)
}

pub fn read_string_int_from_file(file_path: String) -> impl Iterator<Item = (String, i32)> {
    read_file(file_path)
        .lines()
        .filter_map(Result::ok)
        .map(|v| {
            let mut i = v.split(' ');
            (
                i.next().unwrap().to_string(),
                i.next().unwrap().parse::<i32>().unwrap(),
            )
        })
}

pub fn read_int_list_from_file(file_path: String) -> impl Iterator<Item = Vec<u8>> {
    // const RADIX: u32 = 10;
    // c.to_digit(RADIX).unwrap();

    read_file(file_path)
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
}

/*
pub struct IntFileReader {
    file_path: String,
    iter: dyn Iterator<Item = i32>,
}

impl IntFileReader {
    fn new(file_path: String) -> Self {
        // initialize the iterator
        let path = path::Path::new(&file_path);

        let file = match fs::File::open(path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", file_path, why),
            Ok(file) => file,
        };

        let reader = io::BufReader::new(file);

        let iter = reader
            .lines()
            .filter_map(Result::ok)
            .map(|v| v.parse::<i32>())
            .filter_map(Result::ok);

        IntFileReader { file_path, iter }
    }
}

impl Iterator for IntFileReader {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        None
    }
}
*/
/*
pub struct IntFileReader<I>
    where I: Iterator<Item = i32>
{
    file_path: String,
    iter: dyn Iterator<Item = i32>,
}

impl<I> IntFileReader<I>
    where I: Iterator<Item = i32>
{
    pub fn new(file_path: String) {
        // initialize the iterator
        let path = path::Path::new(&file_path);

        let file = match fs::File::open(path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", file_path, why),
            Ok(file) => file,
        };

        let reader = io::BufReader::new(file);

        let iter = reader.lines()
            .filter_map(Result::ok)
            .map(|v| v.parse::<i32>())
            .filter_map(Result::ok);

        IntFileReader{file_path, iter}
    }
}

impl<I> Iterator for IntFileReader<I>
    where I: Iterator<Item = i32>
{
    type Item = I::Item;
    fn next(&mut self) -> Option<i32> {
        self.iter.next()
    }
}
*/

pub struct Triple<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub sum: T,
}

impl Triple<i32> {
    pub fn new(source: Vec<i32>) -> Self {
        assert!(source.len() == 3);
        Triple {
            a: source[0],
            b: source[1],
            c: source[2],
            sum: source[0] + source[1] + source[2],
        }
    }
}

pub struct TripleIter<I>
where
    I: Iterator<Item = i32>,
{
    backlog: VecDeque<i32>,
    iter: I,
}

impl<I> TripleIter<I>
where
    I: Iterator<Item = i32>,
{
    pub fn new(iter: I) -> TripleIter<I> {
        TripleIter {
            backlog: VecDeque::with_capacity(3),
            iter,
        }
    }
}

impl<I> Iterator for TripleIter<I>
where
    I: Iterator<Item = i32>,
{
    // i32
    type Item = I::Item;
    fn next(&mut self) -> Option<i32> {
        // while let Some(entry) = self.iter.next() {
        for entry in &mut self.iter {
            self.backlog.push_front(entry);

            if self.backlog.len() == 3 {
                let value = Triple::new(vec![self.backlog[0], self.backlog[1], self.backlog[2]]);
                self.backlog.pop_back();
                return Some(value.sum);
            }
        }
        // not enough values to continue, so lets clean up
        self.backlog.clear();
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_triples() {
        let records = vec![1, 2, 3, 4];
        // what's up with this copied business ?
        let mut triple = TripleIter::new(records.iter().copied());

        match triple.next() {
            Some(value) => assert_eq!(6, value),
            _ => unreachable!(),
        }

        match triple.next() {
            Some(value) => assert_eq!(9, value),
            _ => unreachable!(),
        }

        assert!(triple.next().is_none());
    }

    #[test]
    fn test_int_list() {
        let mut records = read_int_list_from_file("data/day3.txt".to_string());

        match records.next() {
            Some(r) => assert_eq!(vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], r),
            None => unreachable!(),
        }
    }
}
