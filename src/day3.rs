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
                _ => panic!("unreachable"),
            }
        }
    });
}

pub fn calculate_power(input: Vec<Vec<u8>>) -> (u128, u128, u128) {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    //let mut c0: i32;
    //let mut c1: i32;

    let size: usize = input[0].len();

    let mut v0: Vec<u32> = vec![0; size];
    let mut v1: Vec<u32> = vec![0; size];

    for row in input.iter() {
        for (i, v) in row.iter().enumerate() {
            match v {
                0 => v0[i] += 1,
                1 => v1[i] += 1,
                _ => panic!("unreachable"),
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

fn vec_to_string(input: &Vec<u8>) -> String {
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

    reset_counts(input.iter().collect(), &mut v0, &mut v1);

    let mut o2: Box<Vec<&Vec<u8>>> = Box::new(
        input
            .iter()
            .filter(|row| {
                let want = if v0[0] > v1[0] { 0 } else { 1 };
                if row[0] != want {
                    return false;
                }
                true
            })
            .collect(),
    );

    let mut co2: Box<Vec<&Vec<u8>>> = Box::new(
        input
            .iter()
            .filter(|row| {
                let want = if v0[0] > v1[0] { 1 } else { 0 };
                if row[0] != want {
                    return false;
                }
                true
            })
            .collect(),
    );

    // for v in oxy.iter() {
    //     println!("0: {}", vec_to_string(v));
    // }

    for i in 1..size {
        // loop only until there is one left
        if o2.len() == 1 {
            break;
        }

        reset_counts(o2.to_vec(), &mut v0, &mut v1);

        o2 = Box::new(
            o2.iter()
                .map(|v| *v)
                .filter(|row| {
                    let want = if v0[i] > v1[i] { 0 } else { 1 };
                    // println!("want: {} in ix {}", want, i);
                    if row[i] != want {
                        return false;
                    }
                    true
                })
                .collect(),
        );

        // println!("{}: ", i);
        // for v in oxy.iter() {
        //     println!("{}: {}", i, vec_to_string(v));
        // }
    }
    assert_eq!(o2.len(), 1);
    // println!("OXY: {}", vec_to_string(oxy[0]));

    // for v in c02.iter() {
    //     println!("0: {}", vec_to_string(v));
    // }

    for i in 1..size {
        // loop only until there is one left
        // because of this we need the box, can we avoid it ?
        if co2.len() == 1 {
            break;
        }

        // recalculate the index
        reset_counts(co2.to_vec(), &mut v0, &mut v1);

        co2 = Box::new(
            co2.iter()
                .map(|v| *v)
                .filter(|row| {
                    let want = if v0[i] > v1[i] { 1 } else { 0 };
                    // println!("want: {} in ix {}", want, i);
                    if row[i] != want {
                        return false;
                    }
                    true
                })
                .collect(),
        );

        // println!("{}: ", i);
        // for v in c02.iter() {
        //     println!("{}: {}", i, vec_to_string(v));
        // }
    }
    assert_eq!(co2.len(), 1);
    // println!("CO2: {}", vec_to_string(c02[0]));

    let ov = u128::from_str_radix(&vec_to_string(o2[0]), 2).unwrap();
    let cv = u128::from_str_radix(&vec_to_string(co2[0]), 2).unwrap();
    let rating = ov * cv;
    (ov, cv, rating)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> impl Iterator<Item = Vec<u8>> {
        super::super::util::read_int_list_from_file("data/day3_test.txt".to_string())
    }

    fn puzzle_input() -> impl Iterator<Item = Vec<u8>> {
        super::super::util::read_int_list_from_file("data/day3.txt".to_string())
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
}
