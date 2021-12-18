use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

#[derive(Debug)]
pub enum TargetState {
    Before,
    In,
    Past {
        //before_x: bool,
        after_x: bool,
        //before_y: bool,
        after_y: bool,
    },
}

impl TargetArea {
    fn is_in(&self, position: (i32, i32)) -> TargetState {
        let (x, y) = position;

        if x > self.x.1 || y < self.y.0 {
            //let before_x = x < self.x.0;
            let after_x = x > self.x.1;

            //let before_y = y < self.y.0;
            let after_y = y > self.y.1;

            TargetState::Past {
                //before_x,
                after_x,
                //before_y,
                after_y,
            }
        } else if x < self.x.0 || y > self.y.1 {
            TargetState::Before
        } else {
            TargetState::In
        }
    }
}

pub fn parse(_input: &str) -> TargetArea {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^target area: x=(-?[0-9]+)\.\.(-?[0-9]+), y=(-?[0-9]+)\.\.(-?[0-9]+)$")
                .unwrap();
    }
    // fold along y=7
    // fold along x=5
    if let Some(cap) = RE.captures(_input) {
        assert_eq!(cap.len(), 5);
        TargetArea {
            x: (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            y: (
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
        }
    } else {
        unreachable!()
    }
}

fn check(target: &TargetArea, velocity: &(i32, i32)) -> (TargetState, i32) {
    let (mut x_vel, mut y_vel) = *velocity;

    println!("checking velocity x={}, y={}", x_vel, y_vel);

    let mut position = (0, 0);
    // this will always be before
    let mut position_state = target.is_in(position);

    let mut this_max_y = position.1;

    while let TargetState::Before = position_state {
        position = (position.0 + x_vel, position.1 + y_vel);

        if position.1 > this_max_y {
            this_max_y = position.1;
        }

        if x_vel > 0 {
            x_vel -= 1;
        }
        // we don't have -ve x numbers
        // as if x < 0 then you will never hit the test areas
        // else if x_vel < 0 {
        //    x_vel += 1;
        //}
        y_vel -= 1;

        position_state = target.is_in(position);
        // println!(
        //     "position={:?}, state={:?}, x_vel={}, y_vel={}",
        //     position, position_state, x_vel, y_vel
        // );
    }
    (position_state, this_max_y)
}

pub fn run(target: TargetArea) -> Vec<((i32, i32), i32)> {
    // what is the highest y position on the trajectory
    println!("{:?}", target);

    let mut success = Vec::new();

    'loop_y: for y in -256..256 {
        for x in 1..256 {
            let velocity = (x, y);
            let (result, max_y) = check(&target, &velocity);

            if let TargetState::In = result {
                success.push((velocity, max_y));
            }
            if let TargetState::Past {
                // before_x,
                after_x,
                // before_y,
                after_y,
            } = result
            {
                if after_x && after_y {
                    // we have overshot the mark
                    // more x is not going to help
                    continue 'loop_y;
                }
            }
        }
    }

    success
}

pub fn part1(target: TargetArea) -> i32 {
    *run(target).iter().map(|(_, max_y)| max_y).max().unwrap()
}

pub fn part2(target: TargetArea) -> usize {
    run(target).len()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_input() -> TargetArea {
        parse("target area: x=20..30, y=-10..-5")
    }

    fn puzzle_input() -> TargetArea {
        parse("target area: x=211..232, y=-124..-69")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_input());
        assert_eq!(45, result);

        let result = part1(puzzle_input());
        println!("day 17 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_input());
        assert_eq!(112, result);

        let result = part2(puzzle_input());
        println!("day 17 part 2: result={}", result);
    }
}
