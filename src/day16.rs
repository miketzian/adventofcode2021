use itertools::Itertools;
type DayResult = u128;
use std::str::Chars;

use crate::day16::Packet::*;

pub enum Packet {
    Literal {
        version: u8,
        bin_value: String,
        int_value: u64,
    },
    Operator {
        version: u8,
        packet_type: u8,
        length_type_id: u8,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_count(&self) -> DayResult {
        match self {
            Literal {
                version,
                bin_value: _,
                int_value: _,
            } => *version as u128,
            Operator {
                version,
                packet_type: _,
                length_type_id: _,
                packets,
            } => {
                let mut sub = packets.iter().map(|p| p.version_count()).sum();
                sub += *version as u128;
                sub
            }
        }
    }

    fn value(&self) -> DayResult {
        match self {
            Literal {
                version: _,
                bin_value: _,
                int_value,
            } => *int_value as u128,
            Operator {
                version: _,
                packet_type,
                length_type_id: _,
                packets,
            } => {
                let iter = packets.iter().map(|p| p.value());

                match *packet_type {
                    0 => iter.sum(),
                    1 => iter.product(),
                    2 => iter.min().unwrap(),
                    3 => iter.max().unwrap(),
                    5 => {
                        let (a, b) = iter.collect_tuple().unwrap();
                        if a > b {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        let (a, b) = iter.collect_tuple().unwrap();
                        if a < b {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        let (a, b) = iter.collect_tuple().unwrap();
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn read_literal(version: u8, ci: &mut Chars) -> Option<Packet> {
        // need to have at least one op
        let mut five: String = ci.take(5).collect();
        let mut bin_value = String::new();

        while five.len() == 5 {
            let first = five.chars().take(1).next().unwrap();
            let segment: String = five.chars().skip(1).take(4).collect();
            bin_value.push_str(segment.as_str());

            if first == '0' {
                // this is the last iteration
                break;
            }
            // there is at least one more iteration
            assert_eq!(first, '1');
            five = ci.take(5).collect();
        }

        if !bin_value.is_empty() {
            let int_value = u64::from_str_radix(bin_value.as_str(), 2).unwrap();
            Some(Literal {
                version,
                bin_value,
                int_value,
            })
        } else {
            None
        }
    }

    fn read_operator(version: u8, packet_type: u8, ci: &mut Chars) -> Option<Packet> {
        if let Some(ltid) = ci.next() {
            let length_type_id: u8 = match ltid {
                '0' => 0,
                '1' => 1,
                _ => unreachable!(),
            };

            if length_type_id == 0 {
                let next_15: String = ci.take(15).collect();
                if next_15.len() < 15 {
                    return None;
                }
                let packet_bits = bin_to_usize(next_15);
                let packet_data: String = ci.take(packet_bits as usize).collect();
                let packets = run(packet_data, -1);

                Some(Operator {
                    version,
                    packet_type,
                    length_type_id,
                    packets,
                })
            } else {
                let next_11: String = ci.take(11).collect();
                if next_11.len() < 11 {
                    return None;
                }
                let max_packets = bin_to_usize(next_11);
                let packets = run_chars(ci, max_packets as i32);
                assert_eq!(max_packets as usize, packets.len());

                Some(Operator {
                    version,
                    packet_type,
                    length_type_id,
                    packets,
                })
            }
        } else {
            None
        }
    }
}

fn hex_to_bin(input: String) -> String {
    input
        .trim()
        .chars()
        .fold(String::new(), |mut acc, c| match c {
            '0' => {
                acc.push_str("0000");
                acc
            }
            '1' => {
                acc.push_str("0001");
                acc
            }
            '2' => {
                acc.push_str("0010");
                acc
            }
            '3' => {
                acc.push_str("0011");
                acc
            }
            '4' => {
                acc.push_str("0100");
                acc
            }
            '5' => {
                acc.push_str("0101");
                acc
            }
            '6' => {
                acc.push_str("0110");
                acc
            }
            '7' => {
                acc.push_str("0111");
                acc
            }
            '8' => {
                acc.push_str("1000");
                acc
            }
            '9' => {
                acc.push_str("1001");
                acc
            }
            'A' => {
                acc.push_str("1010");
                acc
            }
            'B' => {
                acc.push_str("1011");
                acc
            }
            'C' => {
                acc.push_str("1100");
                acc
            }
            'D' => {
                acc.push_str("1101");
                acc
            }
            'E' => {
                acc.push_str("1110");
                acc
            }
            'F' => {
                acc.push_str("1111");
                acc
            }
            _ => unreachable!(),
        })
}

fn bin_to_int(input: String) -> u8 {
    u8::from_str_radix(input.as_str(), 2).expect("to parse")
}

fn bin_to_usize(input: String) -> usize {
    usize::from_str_radix(input.as_str(), 2).expect("to parse")
}

fn run(data: String, max_packets: i32) -> Vec<Packet> {
    let mut chars = data.chars();
    let char_iter = chars.by_ref();
    run_chars(char_iter, max_packets)
}

fn try_header(data_iter: &mut Chars) -> Option<(u8, u8)> {
    let v_str: String = data_iter.take(3).collect();
    if v_str.len() < 3 {
        return None;
    }
    let t_str: String = data_iter.take(3).collect();
    if t_str.len() < 3 {
        return None;
    }
    Some((bin_to_int(v_str), bin_to_int(t_str)))
}

fn run_chars(data_iter: &mut Chars, mut max_packets: i32) -> Vec<Packet> {
    let mut packets = Vec::new();
    // let data_iter = data_iter.by_ref();

    let mut maybe_header = try_header(data_iter);

    while let Some((packet_version, packet_type)) = maybe_header {
        if packet_type == 4 {
            if let Some(literal) = Packet::read_literal(packet_version, data_iter) {
                packets.push(literal);
            } else {
                // we have run out of data
                break;
            }
        } else {
            // we have an operator
            if let Some(operator) = Packet::read_operator(packet_version, packet_type, data_iter) {
                packets.push(operator);
            } else {
                // we have run out of data
                break;
            }
        }

        if max_packets > 0 {
            max_packets -= 1;
            if max_packets == 0 {
                break;
            }
        }
        maybe_header = try_header(data_iter);
    }

    packets
}

pub fn part1(_input: String) -> DayResult {
    run(hex_to_bin(_input), -1)
        .iter()
        .map(|p| p.version_count())
        .sum()
}

pub fn part2(_input: String) -> DayResult {
    run(hex_to_bin(_input), -1).iter().map(|p| p.value()).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn puzzle_input() -> String {
        super::super::util::read_file("data/day16.txt")
            .next()
            .expect("one line")
    }

    #[test]
    fn test_part1() {
        assert_eq!(16, part1("8A004A801A8002F478".to_string()));
        assert_eq!(12, part1("620080001611562C8802118E34".to_string()));
        assert_eq!(23, part1("C0015000016115A2E0802F182340".to_string()));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780".to_string()));

        let result = part1(puzzle_input());
        println!("day 16 part 1: result={}", result);
    }

    #[test]
    fn test_part2() {
        assert_eq!(3, part2("C200B40A82".to_string()));
        assert_eq!(54, part2("04005AC33890".to_string()));
        assert_eq!(7, part2("880086C3E88112".to_string()));
        assert_eq!(9, part2("CE00C43D881120".to_string()));
        assert_eq!(1, part2("D8005AC2A8F0".to_string()));
        assert_eq!(0, part2("F600BC2D8F".to_string()));
        assert_eq!(0, part2("9C005AC2F8F0".to_string()));
        assert_eq!(1, part2("9C0141080250320F1802104A08".to_string()));

        let result = part2(puzzle_input());
        println!("day 16 part 2: result={}", result);
    }
}
