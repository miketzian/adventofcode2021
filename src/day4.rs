// type aliases so that the template is easier

type CalculationInput = (Vec<u8>, Vec<BingoCard>);
type DayResult = u64;

// max # is 99

// probably these should be references
// but I haven't read that far in the book
pub struct BingoCard {
    data: Vec<Vec<u8>>,
    checked: Vec<Vec<bool>>,
    winning: Option<Vec<u8>>,
}

impl BingoCard {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        BingoCard {
            data,
            checked: vec![vec![false; 5]; 5],
            winning: None,
        }
    }

    pub fn is_winner(&self) -> bool {
        self.winning.is_some()
    }

    pub fn winner_to_str(&self) -> String {
        if let Some(winner) = self.winning.as_ref() {
            winner
                .iter()
                .map(|i| i.to_string())
                .fold(String::new(), |mut a: String, s: String| {
                    if !a.is_empty() {
                        a.push(' ');
                    }
                    for c in s.chars() {
                        a.push(c);
                    }
                    a
                })
        } else {
            unreachable!()
        }
    }

    /// sum of all unmarked numbers on the board
    pub fn score(&mut self) -> u64 {
        assert!(self.is_winner());

        let mut score: u64 = 0;
        for (i, x_row) in self.data.iter().enumerate() {
            for (j, value) in x_row.iter().enumerate() {
                if !self.checked[i][j] {
                    score += *value as u64;
                }
            }
        }
        score
    }

    pub fn mark_number(&mut self, number: &u8) -> bool {
        // if we know this number, then mark it
        // if the marking results in a win, then return true
        // otherwise, return false.

        for (i, x_row) in self.data.iter().enumerate() {
            for (j, value) in x_row.iter().enumerate() {
                if value == number {
                    // we have just marked!
                    self.checked[i][j] = true;

                    // check that row
                    let row_wins = self.checked[i].iter().all(|v| *v);
                    if row_wins {
                        self.winning = Some(self.data[i].clone());
                        return true;
                    }
                    // check the column

                    let col_wins = (0..5).all(|x| self.checked[x][j]);

                    if col_wins {
                        self.winning = Some((0..5).map(|x| self.data[x][j]).collect());
                        return true;
                    }
                    return false;
                }
            }
        }
        // no number matched
        false
    }
}

struct BingoCardBuilder {
    accum: Vec<Vec<u8>>,
    cards: Vec<Vec<Vec<u8>>>,
    complete: bool,
}

impl BingoCardBuilder {
    pub fn new() -> Self {
        BingoCardBuilder {
            accum: Vec::new(),
            cards: Vec::new(),
            complete: false,
        }
    }

    pub fn read_line(&mut self, input: &str) {
        assert!(!self.complete);

        if input.trim().is_empty() {
            // we should have 5 cards already, or none if it's the first
            if self.accum.len() == 5 {
                self.cards.push(self.accum.clone());
                self.accum = Vec::new();
            } else {
                assert_eq!(0, self.accum.len());
            }
        } else {
            // we should add to an existing card
            self.accum.push(
                input
                    .trim()
                    .replace("  ", " 0")
                    .split(' ')
                    .map(|s| s.parse::<u8>())
                    .filter_map(Result::ok)
                    .collect(),
            );
        }
    }

    pub fn build(&mut self, into: &mut Vec<BingoCard>) {
        self.complete = true;
        for card in self.cards.iter() {
            // why do i need to copy it ? this seems wierd
            let card_data = card.clone();
            into.push(BingoCard::new(card_data));
        }
        if self.accum.len() == 5 {
            // push the last card
            let card_data = self.accum.clone();
            into.push(BingoCard::new(card_data));
        } else {
            // otherwise, we should have no lines
            assert_eq!(0, self.accum.len());
        }
    }
}

pub fn read_blocks_from_file(file_path: &str) -> CalculationInput {
    let mut iter = super::util::read_strings_from_file(file_path.to_string());

    // the first line will be the inputs
    let draw: Vec<u8> = if let Some(first) = iter.next() {
        first
            .trim()
            .split(',')
            .into_iter()
            .map(|v| match v.parse::<u8>() {
                Ok(n) => n,
                Err(_) => unreachable!(),
            })
            .collect()
    } else {
        unreachable!();
    };

    // skip the new line
    iter.next();

    // this function needs to own the cards, can't be given from builder?
    let mut cards = Vec::new();

    iter.fold(BingoCardBuilder::new(), |mut acc, x| {
        acc.read_line(x.as_str());
        acc
    })
    .build(&mut cards);

    // for num in draw.iter() {
    //     println!("Checking {}", num);
    // }
    (draw, cards)
}

pub fn part1(_input: CalculationInput) -> DayResult {
    let (draw, mut cards) = _input;
    for number in draw.iter() {
        println!("Checking {}", number);
        for card in cards.iter_mut() {
            let winner = card.mark_number(number);
            if winner {
                println!("we have a winner! {}", card.winner_to_str());
                return card.score() * (*number as u64);
            }
        }
    }

    unreachable!();
}

pub fn part2(_input: CalculationInput) -> DayResult {
    let (draw, mut cards) = _input;

    let mut has_won: usize = cards.len();

    for number in draw.iter() {
        println!("Checking {}", number);
        // check cards that haven't already won
        for card in cards.iter_mut().filter(|c| !c.is_winner()) {
            // and are now winners
            if card.mark_number(number) {
                // now a winner
                has_won -= 1;
                // the card that was the last to win
                if has_won == 0 {
                    println!("last to win should be ! {}", card.winner_to_str());
                    return card.score() * (*number as u64);
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_data() -> CalculationInput {
        super::read_blocks_from_file("data/day4_test.txt")
    }

    fn puzzle_input() -> CalculationInput {
        super::read_blocks_from_file("data/day4.txt")
    }

    #[test]
    fn test_part1() {
        let result = part1(test_data());

        assert_eq!(4512, result);

        let result = part1(puzzle_input());

        println!("day 4 part 1: {}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_data());

        assert_eq!(1924, result);

        let result = part2(puzzle_input());

        println!("day 4 part 2: {}", result);
    }
}
