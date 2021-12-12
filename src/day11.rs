type CalculationInput = Vec<u8>;
type DayResult = u64;

struct Octopus {
    energy: u8,
    flash_checked: bool,
}

impl Octopus {
    pub fn new(energy: &u8) -> Self {
        Octopus {
            energy: *energy,
            flash_checked: false,
        }
    }

    pub fn incr(&mut self) {
        self.energy += 1;
    }

    pub fn is_flashing(&self) -> bool {
        self.energy > 9
    }

    pub fn newly_flashing(&self) -> bool {
        self.is_flashing() && !self.flash_checked
    }

    pub fn reset(&mut self) {
        self.energy = 0;
        self.flash_checked = false;
    }
}

fn find_neighbors(all: &[Vec<Octopus>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let find = |x: usize, y: usize| {
        if let Some(row) = all.get(x) {
            if row.get(y).is_some() {
                return Some((x, y));
            }
        }
        None
    };
    let mut bors = Vec::new();

    // usize=0 - 1 will panic
    if i > 0 && j > 0 {
        // top left
        if let Some(neighbor) = find(i - 1, j - 1) {
            bors.push(neighbor);
        }
    }

    if i > 0 {
        // left
        if let Some(neighbor) = find(i - 1, j) {
            bors.push(neighbor);
        }
        // bottom left
        if let Some(neighbor) = find(i - 1, j + 1) {
            bors.push(neighbor);
        }
    }
    if j > 0 {
        // right
        if let Some(neighbor) = find(i, j - 1) {
            bors.push(neighbor);
        }
        // top right
        if let Some(neighbor) = find(i + 1, j - 1) {
            bors.push(neighbor);
        }
    }

    if let Some(neighbor) = find(i + 1, j) {
        bors.push(neighbor);
    }
    if let Some(neighbor) = find(i, j + 1) {
        bors.push(neighbor);
    }
    if let Some(neighbor) = find(i + 1, j + 1) {
        bors.push(neighbor);
    }

    bors
}

fn do_round(octopi: &mut Vec<Vec<Octopus>>) -> u64 {
    for pod in octopi.iter_mut() {
        for octopus in pod.iter_mut() {
            octopus.incr();
        }
    }

    let mut do_run = true;

    while do_run {
        do_run = false;
        let mut to_check = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                let mut octopus: &mut Octopus = octopi.get_mut(i).unwrap().get_mut(j).unwrap();
                if octopus.newly_flashing() {
                    octopus.flash_checked = true;
                    to_check.push((i, j));
                }
            }
        }
        for (i, j) in to_check.iter() {
            for (ni, nj) in find_neighbors(octopi, *i, *j) {
                let neighbor = &mut octopi[ni][nj];
                if !neighbor.flash_checked {
                    neighbor.incr();
                    if !do_run {
                        do_run = true;
                    }
                }
            }
        }
    }
    let mut newly_flashing = 0;

    // we check for flashing at the end of the round,
    // and reset
    for x in octopi.iter_mut() {
        for o in x.iter_mut() {
            if o.is_flashing() {
                assert!(o.flash_checked);
                newly_flashing += 1;
                o.reset();
            }
        }
    }

    // octopi.iter().for_each(|row| {
    //     println!(
    //         "{}",
    //         row.iter().fold(String::new(), |mut acc, v| {
    //             for c in v.energy.to_string().chars() {
    //                 acc.push(c);
    //             }
    //             acc
    //         })
    //     );
    // });
    newly_flashing
}

pub fn part1(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let mut octopi: Vec<Vec<Octopus>> = _input
        .map(|v| v.iter().map(Octopus::new).collect())
        .collect();

    let mut flashes: u64 = 0;

    // println!("before start");
    // octopi.iter().for_each(|row| {
    //     println!(
    //         "{}",
    //         row.iter().fold(String::new(), |mut acc, v| {
    //             for c in v.energy.to_string().chars() {
    //                 acc.push(c);
    //             }
    //             acc
    //         })
    //     );
    // });

    for _round in 0..100 {
        // println!("\nRound: {}", _round);
        flashes += do_round(&mut octopi);
    }
    flashes
}

/// in part two, we want to match on the incomplete lines
pub fn part2(_input: impl Iterator<Item = CalculationInput>) -> DayResult {
    let mut octopi: Vec<Vec<Octopus>> = _input
        .map(|v| v.iter().map(Octopus::new).collect())
        .collect();

    let just_flashed =
        |oo: &Vec<Vec<Octopus>>| oo.iter().all(|row| row.iter().all(|o| o.energy == 0));

    let mut round: u64 = 0;
    while !just_flashed(&octopi) {
        round += 1;
        do_round(&mut octopi);
        assert!(round < 250);
    }

    round
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_int_list_from_file("data/day11_test.txt")
    }

    fn puzzle_input() -> impl Iterator<Item = CalculationInput> {
        super::super::util::read_int_list_from_file("data/day11.txt")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(1656, result);

        let result = part1(puzzle_input());
        println!("day 11 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(195, result);

        let result = part2(puzzle_input());

        println!("day 11 part 2: result={}", result);
    }
}
