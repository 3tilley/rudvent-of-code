use rand::rngs::ThreadRng;
use rand::Rng;
use rudvent_lib::solution::execution::{
    EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor,
};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Map;
use std::slice::Iter;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Hand>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 6440;
const EXAMPLE_2_ANS: OutputPart2 = 5905;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
pub struct Hand {
    _hand_type: Cell<Option<HandType>>,
    cards: [u8; 5],
    joker_cards: Cell<Option<[u8; 5]>>,
    bid: usize,
    jokers: bool,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        info!("{:?}", self.cards);
        match &self._hand_type.get() {
            None => {
                let mut counter = HashMap::<u8, u8>::with_capacity(5);
                for card in self.cards {
                    counter.entry(card).and_modify(|v| *v += 1).or_insert(1);
                }
                info!("{:?}", counter);
                let hand_type = match counter.len() {
                    1 => HandType::FiveOfAKind,
                    5 => HandType::HighCard,
                    4 => HandType::OnePair,
                    2 => {
                        if counter.iter().any(|(k, v)| v == &4) {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    }
                    3 => {
                        if counter.iter().any(|(k, v)| v == &3) {
                            HandType::ThreeOfAKind
                        } else {
                            HandType::TwoPair
                        }
                    }
                    _ => unreachable!("Not all possibilities accounted for"),
                };
                self._hand_type.replace(Some(hand_type));
                hand_type
            }
            Some(t) => *t,
        }
    }
    fn hand_type_jokers(&self) -> HandType {
        info!("{:?}", self.cards);
        match &self._hand_type.get() {
            None => {
                let mut counter = HashMap::<u8, u8>::with_capacity(5);
                for card in self.cards {
                    counter.entry(card).and_modify(|v| *v += 1).or_insert(1);
                }
                let (_, mut others): (Vec<u8>, Vec<u8>) =
                    self.cards.iter().partition(|&&c| c == 11);
                let joker_count = counter.remove(&11).unwrap_or(0);
                let highest_card = others.iter().max();

                info!("{:?}", counter);
                // 1 - 5ok
                // 2:
                //      2-2 - FH    - 1J
                //      3-1 - 4ok   - 1J
                //      2-1 - 4ok   - 2J
                //      1-1 - 4ok   - 3J - Watch for highest
                // 3:
                //      1-1-1 - 3ok - 2J
                //      2-1-1 - 3ok - 1J
                // 4:
                //      1-1-1-1 - 2ok - 1J
                let (hand_type, cards) = match counter.len() {
                    0 => (HandType::FiveOfAKind, vec![14, 14, 14, 14, 14]),
                    1 => (HandType::FiveOfAKind, vec![others[0]; 5]),
                    2 => {
                        if counter.iter().any(|(k, v)| v == &1) {
                            if joker_count == 3 {
                                (
                                    HandType::FourOfAKind,
                                    self.replace_jokers(*highest_card.unwrap()),
                                )
                            } else {
                                let multi = counter.iter().find(|(&k, &v)| v != 1).unwrap().0;
                                (HandType::FourOfAKind, self.replace_jokers(*multi))
                            }
                        } else {
                            (
                                HandType::FullHouse,
                                self.replace_jokers(*highest_card.unwrap()),
                            )
                        }
                    }
                    3 => {
                        if joker_count == 0 {
                            if counter.iter().any(|(k, v)| v == &3) {
                                (HandType::ThreeOfAKind, self.cards.into_iter().collect())
                            } else {
                                (HandType::TwoPair, self.cards.into_iter().collect())
                            }
                        } else {
                            let multi = counter.iter().find(|(&k, &v)| v != 1);
                            match multi {
                                None => (
                                    HandType::ThreeOfAKind,
                                    self.replace_jokers(*highest_card.unwrap()),
                                ),
                                Some(mult) => {
                                    (HandType::ThreeOfAKind, self.replace_jokers(*mult.0))
                                }
                            }
                        }
                    }
                    4 => (
                        HandType::OnePair,
                        self.replace_jokers(*highest_card.unwrap()),
                    ),
                    5 => (HandType::HighCard, others),
                    _ => unreachable!("Not all possibilities accounted for"),
                };
                self._hand_type.replace(Some(hand_type));
                self.joker_cards.replace(Some(cards.try_into().unwrap()));
                hand_type
            }
            Some(t) => *t,
        }
    }

    fn replace_jokers(&self, replace_with: u8) -> Vec<u8> {
        self.cards
            .iter()
            .map(move |c| if *c == 11 { replace_with } else { *c })
            .collect()
    }

    fn from_str(s: &str, jokers: bool) -> Hand {
        let (card_str, bid_str) = s.split_once(" ").unwrap();
        let mut card_iter = card_str.chars().map(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap() as u8
            } else {
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!("Unrecognised digit"),
                }
            }
        });
        let cards = [
            card_iter.next().unwrap(),
            card_iter.next().unwrap(),
            card_iter.next().unwrap(),
            card_iter.next().unwrap(),
            card_iter.next().unwrap(),
        ];
        let bid = usize::from_str(bid_str).unwrap();
        Hand {
            _hand_type: Cell::new(None),
            cards,
            bid,
            joker_cards: Cell::new(None),
            jokers,
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.jokers {
            (self.hand_type_jokers(), &self.cards) == (other.hand_type_jokers(), &other.cards)
        } else {
            (self.hand_type(), self.cards) == (other.hand_type(), other.cards)
        }
    }
}

// This works because it tells the compiler that it can use PartialEq and treat it as full Eq
impl Eq for Hand {}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.jokers {
            match self.hand_type_jokers().cmp(&other.hand_type_jokers()) {
                Ordering::Equal => self.cards.cmp(&other.cards),
                x => x,
            }
        } else {
            match self.hand_type().cmp(&other.hand_type()) {
                Ordering::Equal => self.cards.cmp(&other.cards),
                x => x,
            }
        }
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input
        .lines()
        .map(|line| Hand::from_str(line, false))
        .collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.sort();
    info!("{:?}", input);
    input.iter().enumerate().map(|(i, v)| (i + 1) * v.bid).sum()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    input
        .lines()
        .map(|line| Hand::from_str(line, true))
        .collect()
}

fn make_random_non_joker(rng: &mut ThreadRng) -> u8 {
    let out = rng.gen_range(2..=13);
    if out >= 11 {
        out + 1
    } else {
        out
    }
}

fn check_part_2(input: &InputPart1) -> usize {
    let mut rng = rand::thread_rng();
    let mut count_errors = 0;
    input.iter().for_each(|hand| {
        for i in 0..100 {
            let new_hand = Hand {
                _hand_type: Cell::new(None),
                cards: hand.cards.map(|c| {
                    if c == 11 {
                        make_random_non_joker(&mut rng)
                    } else {
                        c
                    }
                }),
                joker_cards: Cell::new(None),
                bid: 1,
                jokers: true,
            };
            if &new_hand == hand {
                info!("Hands are equal");
            } else if &new_hand < hand {
                info!("Joker hand is better");
            } else {
                error!(
                    "New hand beats joker hand {:?} - Joker {:?} - Joker replaced: {:?}",
                    new_hand.cards,
                    hand.cards,
                    hand.joker_cards.get()
                );

                error!("New hand: {:?}", new_hand);
                error!("Joker hand: {:?}", hand);
                count_errors += 1;
            }
            info!("New hand: {:?}", new_hand);
            info!("Joker hand: {:?}", hand);
            // assert_eq!(new_hand, *hand);
        }
    });
    count_errors
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let errors = check_part_2(&input);
    println!("{} errors", errors);

    part_1(input, run_parameter, monitor)
}

// ----- There is no need to change anything below this line -----
// The below code creates a solution that is generic over several types. These types might change
// between different days, for example integers on some and strings on others. They are type-aliased
// above to make it easier to change them all at once
pub fn make_sol() -> Box<dyn SolutionBuilder> {
    let sol = StructSolutionBuilder::new(
        prepare,
        part_1,
        prepare_2,
        part_2,
        Example::Value(EXAMPLE_1_ANS),
        Example::Value(EXAMPLE_2_ANS),
    );
    Box::new(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_ordering() {
        let high_type = HandType::FiveOfAKind;
        let low_type = HandType::HighCard;
        assert!(high_type > low_type)
    }

    #[test]
    fn test_array_ordering() {
        let arr_1 = [4, 3, 2];
        let arr_2 = [3, 4, 5];
        assert!(arr_1 > arr_2);
    }

    #[test]
    fn test_equal_jokers() {
        let joker_4ok = Hand::from_str("JKKK2 1", true);
        let other_4ok = Hand::from_str("QQQQ2 1", true);
        assert!(other_4ok > joker_4ok);
        assert_ne!(other_4ok, joker_4ok);
    }
}
