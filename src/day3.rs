fn reset_counts(input: Vec<&Vec<u8>>, v0: &mut Vec<u32>, v1: &mut Vec<u32>) {
    for i in 0..v0.len() {
        v0[i] = 0;
        v1[i] = 0;
    }

    input.iter().for_each(|row| {
        for (i, v) in row.iter().enumerate() {
            match v {
                0 => v0[i] += 1,
                1 => v1[i] += 1,
                _ => unreachable!(),
            }
        }
    });
}

pub fn calculate_power(input: Vec<Vec<u8>>) -> (u128, u128, u128) {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    let size: usize = input[0].len();

    let mut v0: Vec<u32> = vec![0; size];
    let mut v1: Vec<u32> = vec![0; size];

    for row in input.iter() {
        for (i, v) in row.iter().enumerate() {
            match v {
                0 => v0[i] += 1,
                1 => v1[i] += 1,
                _ => unreachable!(),
            }
        }
    }

    for i in 0..size {
        if v0[i] > v1[i] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gv = u128::from_str_radix(&gamma, 2).unwrap();
    let ev = u128::from_str_radix(&epsilon, 2).unwrap();
    let power = gv * ev;
    (gv, ev, power)
}

fn vec_to_string(input: &[u8]) -> String {
    let mut output: String = String::new();
    for v in input.iter() {
        match v {
            0 => output.push('0'),
            1 => output.push('1'),
            _ => unreachable!(),
        }
    }
    output
}

pub fn calc_generators(input: Vec<Vec<u8>>) -> (u128, u128, u128) {
    let size: usize = input[0].len();

    let mut v0: Vec<u32> = vec![0; size];
    let mut v1: Vec<u32> = vec![0; size];

    // need a vector of references
    reset_counts(input.iter().collect(), &mut v0, &mut v1);

    let mut o2: Vec<&Vec<u8>> = input
        .iter()
        .filter(|row| {
            let want = if v0[0] > v1[0] { 0 } else { 1 };
            if row[0] != want {
                return false;
            }
            true
        })
        .collect();

    let mut co2: Vec<&Vec<u8>> = input
        .iter()
        .filter(|row| {
            let want = if v0[0] > v1[0] { 1 } else { 0 };
            if row[0] != want {
                return false;
            }
            true
        })
        .collect();

    for i in 1..size {
        // loop only until there is one left

        if o2.len() == 1 {
            break;
        }

        reset_counts(o2.to_vec(), &mut v0, &mut v1);

        o2 = o2
            .iter()
            .cloned()
            .filter(|row| {
                let want = if v0[i] > v1[i] { 0 } else { 1 };
                if row[i] != want {
                    return false;
                }
                true
            })
            .collect();
    }

    assert_eq!(o2.len(), 1);

    for i in 1..size {
        if co2.len() == 1 {
            break;
        }

        // recalculate the index
        reset_counts(co2.to_vec(), &mut v0, &mut v1);

        co2 = co2
            .iter()
            .cloned()
            .filter(|row| {
                let want = if v0[i] > v1[i] { 1 } else { 0 };
                // println!("want: {} in ix {}", want, i);
                if row[i] != want {
                    return false;
                }
                true
            })
            .collect();
    }
    assert_eq!(co2.len(), 1);

    let ov = u128::from_str_radix(&vec_to_string(o2[0]), 2).unwrap();
    let cv = u128::from_str_radix(&vec_to_string(co2[0]), 2).unwrap();
    let rating = ov * cv;
    (ov, cv, rating)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = Vec<u8>> {
        super::super::util::read_int_list_from_file("data/day3_test.txt")
    }

    fn puzzle_input() -> impl Iterator<Item = Vec<u8>> {
        super::super::util::read_int_list_from_file("data/day3.txt")
    }

    #[test]
    fn test_calc_gamma() {
        let (gamma, epsilon, power) = calculate_power(test_data().collect());
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(power, 198);

        let (gamma, epsilon, power) = calculate_power(puzzle_input().collect());

        println!("gamma={}, epsilon={}, power={}", gamma, epsilon, power);
    }

    #[test]
    fn test_calc_generators() {
        let (oxy, co2, rating) = calc_generators(test_data().collect());

        assert_eq!(oxy, 23);
        assert_eq!(co2, 10);
        assert_eq!(rating, 230);

        let (oxy, co2, rating) = calc_generators(puzzle_input().collect());
        println!("o2={}, c02={}, power={}", oxy, co2, rating);
    }

    #[test]
    #[should_panic]
    fn test_reset_counts() {
        let mut v0: Vec<u32> = vec![0; 2];
        let mut v1: Vec<u32> = vec![0; 2];

        let row = &vec![1, 2]; // 2 is invalid
        let data = vec![row];

        reset_counts(data, &mut v0, &mut v1);
    }
}
