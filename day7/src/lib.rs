// Jack, queen, king, ace values
const J: usize = 11;
const Q: usize = 12;
const K: usize = 13;
const A: usize = 14;

pub fn day_7_1(text: &str) -> u32 {
    // Return the total winnings from of set of "Camel Cards" (poker) hands
    // Note: original ordering is important for Camel Cards tie-breakers
    let mut hands: Vec<&str> = vec![];
    let mut bids: Vec<u32> = vec![];

    for (i, line) in text.lines().enumerate() {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands.push(hand);
        bids.push(bid.parse::<u32>().unwrap_or_default());
    }
    // Determine hand type, assign relative values (0 to hands.len() for worst..best hand)
    let hand_values = assign_hand_values(hands);

    return 0;
}

fn assign_hand_values(hands: Vec<&str>) -> Vec<u32> {
    // Return vec of relative hand values, ordered with respect to the input vec
    let mut hist: [u32; A as usize + 1];
    for (i, hand) in hands.iter().enumerate() {
        hist = [0; A as usize + 1];
        for c in hand.chars() {
            if c.is_digit(10) {
                hist[c.to_digit(10).unwrap() as usize] += 1;
            } else {
                match c {
                'J' => hist[J] += 1,
                'Q' => hist[Q] += 1,
                'K' => hist[K] += 1,
                'A' => hist[A] += 1,
                _ => panic!("found invalid char {c}"),
                }
            }
        }
        // No straights or flushes in Camel Cards
        if hist.contains(&5) {
            // 5 of a kind
        } else if hist.contains(&4) {
            // 4 of a kind
        } else if hist.contains(&3) {
            // 3 of a kind, maybe full house
        } else if hist.contains(&2) {
            // pair, maybe two pair
        } else {
            // high card
        }
    }
    return vec![hands.len() as u32; 0];
}
/*
match c.parse() {
    Ok(c) => {
        hist[c] += 1;
    }
    Err(e) => {
    }
}
*/
