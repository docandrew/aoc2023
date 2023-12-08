use std::fs;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Given the card character, return the value of the card.
fn card_val(c : char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
         _   => c.to_digit(10).unwrap() as u8,
    }
}

// Given the card character (with Jokers wild), return the value of the card.
fn card_val2(c : char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
         _   => c.to_digit(10).unwrap() as u8,
    }
}

#[derive(Debug)]
struct Hand {
    hand_str: String,
    hand_type: HandType,
    bid: u32,
}

fn cmp(a : &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.hand_type > b.hand_type {
        return std::cmp::Ordering::Greater;
    } else if a.hand_type < b.hand_type {
        return std::cmp::Ordering::Less;
    } else {
        // same type of hand, need to compare individual cards now.
        // iterate until we find one worth more than its counterpart
        for i in 0..a.hand_str.len() {
            let self_val = card_val(a.hand_str.chars().nth(i).unwrap());
            let other_val = card_val(b.hand_str.chars().nth(i).unwrap());
            if self_val < other_val {
                return std::cmp::Ordering::Less;
            } else if self_val > other_val {
                return std::cmp::Ordering::Greater;
            }
        }

        // if we get here, the hands are identical.
        return std::cmp::Ordering::Equal;
    }
}

// Compare hands with Jokers wild
fn cmp2(a : &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.hand_type > b.hand_type {
        return std::cmp::Ordering::Greater;
    } else if a.hand_type < b.hand_type {
        return std::cmp::Ordering::Less;
    } else {
        // same type of hand, need to compare individual cards now.
        // iterate until we find one worth more than its counterpart
        for i in 0..a.hand_str.len() {
            let self_val = card_val2(a.hand_str.chars().nth(i).unwrap());
            let other_val = card_val2(b.hand_str.chars().nth(i).unwrap());

            if self_val < other_val {
                return std::cmp::Ordering::Less;
            } else if self_val > other_val {
                return std::cmp::Ordering::Greater;
            }
        }

        // if we get here, the hands are identical.
        return std::cmp::Ordering::Equal;
    }
}

// given a string with a particular hand, determine the hand type.
fn hand_type(hand_str : &str) -> HandType {
    let mut card_counts : HashMap<char, u8> = HashMap::new();

    for c in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
        card_counts.insert(c, 0);
    }
    // populate card counts
    for c in hand_str.chars() {
        card_counts.insert(c, card_counts.get(&c).unwrap() + 1);
    }

    // determine hand type
    for c in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
        if *card_counts.get(&c).unwrap() == 5 {
            return HandType::FiveOfAKind;
        } else if *card_counts.get(&c).unwrap() == 4 {
            return HandType::FourOfAKind;
        } else if *card_counts.get(&c).unwrap() == 3 {
            // check for full house
            for c2 in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                if *card_counts.get(&c2).unwrap() == 2 {
                    return HandType::FullHouse;
                }
            }
            return HandType::ThreeOfAKind;
        } else if *card_counts.get(&c).unwrap() == 2 {
            // check for full house
            for c2 in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                if *card_counts.get(&c2).unwrap() == 3 {
                    return HandType::FullHouse;
                }
            }
            // check for two pair
            for c2 in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                if c2 != c && *card_counts.get(&c2).unwrap() == 2 {
                    return HandType::TwoPair;
                }
            }
            return HandType::OnePair;
        }
    }

    return HandType::HighCard;
}

// Jokers wild version
fn hand_type2(hand_str : &str) -> HandType {
    // determine hand type with jokers _removed_
    let hand_str2_binding = hand_str.replace("J","");
    let hand_str2 = hand_str2_binding.as_str();
    let hand_type_no_joker = hand_type(hand_str2);
    let joker_count = hand_str.len() - hand_str2.len();

    if joker_count == 5 {
        return HandType::FiveOfAKind;
    } else if joker_count == 4 {
        return HandType::FiveOfAKind;
    } else if joker_count == 3 {
        if hand_type_no_joker == HandType::OnePair {
            return HandType::FiveOfAKind;
        } else {
            return HandType::FourOfAKind;
        }
    } else if joker_count == 2 {
        if hand_type_no_joker == HandType::ThreeOfAKind {
            // deck had 3 cards that matched
            return HandType::FiveOfAKind;
        } else if hand_type_no_joker == HandType::OnePair {
            // deck had 2 matching cards
            return HandType::FourOfAKind;
        } else {
            // deck had 3 other non-matching cards
            return HandType::ThreeOfAKind;
        }
    } else if joker_count == 1 {
        if hand_type_no_joker == HandType::FourOfAKind {
            // deck had 4 cards that matched
            return HandType::FiveOfAKind;
        } else if hand_type_no_joker == HandType::ThreeOfAKind {
            // deck had 3 cards that matched
            return HandType::FourOfAKind;
        } else if hand_type_no_joker == HandType::TwoPair {
            return HandType::FullHouse;
        } else if hand_type_no_joker == HandType::OnePair {
            // deck had 2 matching cards
            return HandType::ThreeOfAKind;
        } else {
            // deck had 4 other non-matching cards,
            // we can make a pair.
            return HandType::OnePair;
        }
    } else {
        // no jokers, hand is what it was
        return hand_type_no_joker;
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");

    let mut hands : Vec<Hand> = Vec::new();

    for line in binding.lines() {
        let parts : Vec<&str> = line.split_whitespace().collect();
        let hand_str = parts[0];
        let bid_str  = parts[1];

        let hand = Hand {
            hand_str: hand_str.to_string(),
            hand_type: hand_type(hand_str),
            bid: bid_str.parse::<u32>().unwrap(),
        };

        hands.push(hand);
    }

    hands.sort_by(|a, b| cmp(a, b));

    let mut total_score = 0;

    for i in 0..hands.len() {
        let hand = &hands[i];
        let rank = i+1;
        let score = hand.bid * rank as u32;
        total_score += score;
        println!("Hand: {}, type: {:?}, rank: {}, bid: {}, score: {}", hand.hand_str, hand.hand_type, rank, hand.bid, score);
    }

    println!("Part 1 Answer: {}", total_score);

    let mut hands2 : Vec<Hand> = Vec::new();

    for line in binding.lines() {
        let parts : Vec<&str> = line.split_whitespace().collect();
        let hand_str = parts[0];
        let bid_str  = parts[1];

        let hand = Hand {
            hand_str: hand_str.to_string(),
            hand_type: hand_type2(hand_str),
            bid: bid_str.parse::<u32>().unwrap(),
        };

        hands2.push(hand);
    }

    // re-sort with Jokers wild rules
    hands2.sort_by(|a, b| cmp2(a, b));
  
    total_score = 0;
    
    for i in 0..hands2.len() {
        let hand = &hands2[i];
        let rank = i+1;
        let score = hand.bid * rank as u32;
        total_score += score;
        println!("Hand: {}, type: {:?}, rank: {}, bid: {}, score: {}", hand.hand_str, hand.hand_type, rank, hand.bid, score);
    }

    println!("Part 2 Answer: {}", total_score);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_card_val() {
        assert_eq!(super::card_val('A'), 14);
        assert_eq!(super::card_val('K'), 13);
        assert_eq!(super::card_val('2'), 2);
    }
    
    #[test]
    fn test_hand_type() {
        assert_eq!(super::hand_type("AAAAA"), super::HandType::FiveOfAKind);
        assert_eq!(super::hand_type("22255"), super::HandType::FullHouse);
        assert_eq!(super::hand_type("22AA2"), super::HandType::FullHouse);
    }

    #[test]
    fn test_hand_type2() {
        assert_eq!(super::hand_type2("AAAAA"), super::HandType::FiveOfAKind);
        assert_eq!(super::hand_type2("22255"), super::HandType::FullHouse);
        assert_eq!(super::hand_type2("22J55"), super::HandType::FullHouse);
        assert_eq!(super::hand_type2("22AJJ"), super::HandType::FourOfAKind);
        assert_eq!(super::hand_type2("32T3K"), super::HandType::OnePair);
        assert_eq!(super::hand_type2("T55J5"), super::HandType::FourOfAKind);
    }
}