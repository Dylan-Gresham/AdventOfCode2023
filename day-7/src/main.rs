use std::fs;

#[derive(PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u8 {
        match *self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

struct Hand<'a> {
    cards: &'a str,
    bid: u16,
    hand_type: HandType,
}

impl Hand<'_> {
    fn new(cards: &'static str, bid: u16) -> Self {
        let mut card_labels: Vec<String> = vec![];
        let mut labels_frequency: Vec<u8> = vec![];

        for idx in 0..cards.len() {
            let curr_char = cards.chars().nth(idx).unwrap().to_string();

            if card_labels.contains(&curr_char) {
                let char_idx = card_labels
                    .iter()
                    .position(|label| label == &curr_char)
                    .unwrap();
                labels_frequency[char_idx] += 1;
            } else {
                card_labels.push(curr_char);
                labels_frequency.push(1);
            }
        }

        let num_cards: u8 = card_labels.len().try_into().unwrap();

        if num_cards == 1 {
            Self {
                cards,
                bid,
                hand_type: HandType::FiveOfAKind,
            }
        } else if num_cards == 2 {
            if labels_frequency.contains(&4) {
                Self {
                    cards,
                    bid,
                    hand_type: HandType::FourOfAKind,
                }
            } else {
                Self {
                    cards,
                    bid,
                    hand_type: HandType::FullHouse,
                }
            }
        } else if num_cards == 3 {
            if labels_frequency.contains(&3) {
                Self {
                    cards,
                    bid,
                    hand_type: HandType::ThreeOfAKind,
                }
            } else {
                Self {
                    cards,
                    bid,
                    hand_type: HandType::TwoPair,
                }
            }
        } else if num_cards == 4 {
            Self {
                cards,
                bid,
                hand_type: HandType::OnePair,
            }
        } else {
            Self {
                cards,
                bid,
                hand_type: HandType::HighCard,
            }
        }
    }

    fn compare_hands(hand_one: Hand, hand_two: Hand) -> i8 {
        if hand_one.hand_type > hand_two.hand_type {
            1
        } else if hand_one.hand_type < hand_two.hand_type {
            -1
        } else {
            let labels: Vec<&str> = vec![
                "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
            ];
            for i in 0..5 {
                let hand_one_str = hand_one.cards.chars().nth(i).unwrap().to_string();
                let hand_two_str = hand_two.cards.chars().nth(i).unwrap().to_string();
                let hand_one_idx = labels.iter().position(|&r| r == hand_one_str).unwrap();
                let hand_two_idx = labels.iter().position(|&r| r == hand_two_str).unwrap();

                if hand_one_idx < hand_two_idx {
                    return 1;
                } else if hand_one_idx > hand_two_idx {
                    return -1;
                }
            }

            0
        }
    }
}

fn part_one() -> u32 {
    let file_string = fs::read_to_string("./test-input.txt").expect("File should exist");
    let file: &'static str = &file_string.clone();
    let lines = file.lines().collect::<Vec<&str>>();

    let mut hands: Vec<Hand> = vec![];
    for line in lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hand: &str = parts[0];
        let bid: u16 = parts[1].parse::<u16>().unwrap();
        hands.push(Hand::new(hand, bid));
    }

    0
}

fn main() {
    println!("Total for part one: {}", part_one());
}
