// Jack, queen, king, ace values
const T: u32 = 10;
const Q: u32 = 12;
const K: u32 = 13;
const A: u32 = 14;

const FIVE_OF_A_KIND: u32 = 6;
const FOUR_OF_A_KIND: u32 = 5;
const FULL_HOUSE: u32 = 4;
const THREE_OF_A_KIND: u32 = 3;
const TWO_PAIR: u32 = 2;
const PAIR: u32 = 1;
const HIGH_CARD: u32 = 0;

pub fn day_7_1(text: &str) -> u32 {
    // Return the total winnings from of set of "Camel Cards" (poker) hands
    const J: u32 = 11;
    let mut hands: Vec<[u32; 5]> = vec![];
    let mut bids: Vec<u32> = vec![];

    for line in text.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        bids.push(bid.parse::<u32>().unwrap_or_default());

        // Can this be done in one chain of function calls, ...collect()?
        let hand: Vec<char> = hand.chars().collect::<Vec<char>>();
        let mut cards_in_hand: [u32; 5] = [0; 5];
        for (i, card) in hand.iter().enumerate() {
            if card.is_digit(10) {
                cards_in_hand[i] = card.to_digit(10).unwrap();
            } else {
                match card {
                    'T' => cards_in_hand[i] = T,
                    'J' => cards_in_hand[i] = J,
                    'Q' => cards_in_hand[i] = Q,
                    'K' => cards_in_hand[i] = K,
                    'A' => cards_in_hand[i] = A,
                    _ => panic!("invalid char {card}"),
                }
            }
        }
        hands.push(cards_in_hand);
    }
    // Determine hand type, assign relative values (1 to hands.len() for worst..best hand)
    let rankings = rank_hands(&hands);

    let mut total_winnings = 0;
    for (i, rank) in rankings.iter().enumerate() {
        total_winnings += rank * bids[i];
    }

    return total_winnings as u32;
}

pub fn day_7_2(text: &str) -> u32 {
    // Return the total winnings from of set of "Camel Cards" (poker) hands
    // J cards are now jokers, not jacks
    const J: u32 = 1;
    let mut hands: Vec<[u32; 5]> = vec![];
    let mut bids: Vec<u32> = vec![];

    for line in text.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        bids.push(bid.parse::<u32>().unwrap_or_default());

        // Can this be done in one chain of function calls, ...collect()?
        let hand: Vec<char> = hand.chars().collect::<Vec<char>>();
        let mut cards_in_hand: [u32; 5] = [0; 5];
        for (i, card) in hand.iter().enumerate() {
            if card.is_digit(10) {
                cards_in_hand[i] = card.to_digit(10).unwrap();
            } else {
                match card {
                    'T' => cards_in_hand[i] = T,
                    'J' => cards_in_hand[i] = J,
                    'Q' => cards_in_hand[i] = Q,
                    'K' => cards_in_hand[i] = K,
                    'A' => cards_in_hand[i] = A,
                    _ => panic!("invalid char {card}"),
                }
            }
        }
        hands.push(cards_in_hand);
    }
    // Determine hand type, assign relative values (1 to hands.len() for worst..best hand)
    let rankings = rank_hands(&hands);

    let mut total_winnings = 0;
    for (i, rank) in rankings.iter().enumerate() {
        total_winnings += rank * bids[i];
    }

    return total_winnings as u32;
}

fn rank_hands(hands: &Vec<[u32; 5]>) -> Vec<u32> {
    // Return vec of hand rankings, ordered with respect to the vec arg
    let mut hand_types: Vec<u32> = vec![0; hands.len()];
    let mut hist: [u32; A as usize + 1];

    for (i, hand) in hands.iter().enumerate() {
        hist = [0; A as usize + 1];
        for j in 0..hand.len() {
            hist[hand[j] as usize] += 1;
        }

        // count_jokers will always be 0 for day_7_1()
        let count_jokers = hist[1];
        hist[1] = 0;
        let mut max = 0;
        let mut max_ind = 0;
        if count_jokers > 0 {
            for i in 0..hist.len() {
                if hist[i] > max {
                    max = hist[i];
                    max_ind = i;
                }
            }
        }
        hist[max_ind] += count_jokers;

        // No straights or flushes in Camel Cards
        let mut count_2s = 0;
        let mut count_3s = 0;
        for j in 0..hist.len() {
            if hist[j] == 5 {
                hand_types[i] = FIVE_OF_A_KIND;
                break;
            } else if hist[j] == 4 {
                hand_types[i] = FOUR_OF_A_KIND;
                break;
            } else if hist[j] == 3 {
                count_3s += 1;
            } else if hist[j] == 2 {
                count_2s += 1;
            }
        }
        if hand_types[i] > 0 {
            continue;
        }
        assert_eq!(hist.iter().sum::<u32>(), 5);
        if count_3s == 1 {
            if count_2s == 1 {
                hand_types[i] = FULL_HOUSE;
            } else {
                hand_types[i] = THREE_OF_A_KIND;
            }
        } else if count_2s > 0 {
            if count_2s == 2 {
                hand_types[i] = TWO_PAIR;
            } else {
                hand_types[i] = PAIR;
            }
        } else {
            hand_types[i] = HIGH_CARD;
        }
    }
    // Pseudo-base conversion. Each card value occupies two digits of the number
    // The card values do not overlap each other's digits
    let mut hand_values: Vec<(usize, u32)> = Vec::with_capacity(hands.len());

    let base: u32 = 100;
    for i in 0..hands.len() {
        let mut num = 0;
        for j in 0..hands[0].len() {
            num += hands[i][j] * (100_000_000 / base.pow(j as u32));
        }
        hand_values.push((i, num));
    }

    // Assign hand rankings, from low (1) to high (hands.len())
    let mut ranking_to_assign = 1;
    let mut rankings: Vec<u32> = vec![0; hands.len()];
    let mut tied: Vec<(usize, u32)> = vec![];

    for type_val in HIGH_CARD..=FIVE_OF_A_KIND {
        tied.clear();
        for j in 0..hand_types.len() {
            if hand_types[j] == type_val {
                tied.push(hand_values[j]);
            }
        }
        if tied.len() == 1 {
            rankings[tied[0].0] = ranking_to_assign;
            ranking_to_assign += 1;
            continue;
        }
        if tied.len() == 0 {
            continue;
        }

        // Break ties
        tied.sort_by(|a, b| a.1.cmp(&b.1));
        for (hand_ind, _) in &tied {
            rankings[*hand_ind] = ranking_to_assign;
            ranking_to_assign += 1;
        }
    }

    return rankings;
}
