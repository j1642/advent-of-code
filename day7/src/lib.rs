// Jack, queen, king, ace values
const T: u32 = 10;
const J: u32 = 11;
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
    // Determine hand type, assign relative values (0 to hands.len() for worst..best hand)
    let hand_values = assign_hand_values(hands);

    let mut total_winnings = 0;
    for (i, hand_value) in hand_values.iter().enumerate() {
        total_winnings += (hand_value.0 as u32 + 1) * bids[i];
        let bid = bids[i];
        println!("{}, {}", hand_value.0 + 1, bid);
    }

    return total_winnings as u32;
}

fn assign_hand_values(hands: Vec<[u32; 5]>) -> Vec<(usize, u32)> {
    // Return vec of relative hand values, ordered with respect to the input vec
    let mut absolute_hand_values: Vec<u32> = vec![0; hands.len()];
    let mut hist: [u32; A as usize + 1];

    for (i, hand) in hands.iter().enumerate() {
        hist = [0; A as usize + 1];
        for j in 0..hand.len() {
            hist[hand[j] as usize] += 1;
        }
        // No straights or flushes in Camel Cards
        let mut count_2s = 0;
        let mut count_3s = 0;
        for j in 0..hist.len() {
            if hist[j] == 5 {
                absolute_hand_values[i] = FIVE_OF_A_KIND;
                break;
            } else if hist[j] == 4 {
                absolute_hand_values[i] = FOUR_OF_A_KIND;
                break;
            } else if hist[j] == 3 {
                count_3s += 1;
            } else if hist[j] == 2 {
                count_2s += 1;
            }
        }
        assert_eq!(hist.iter().sum::<u32>(), 5);
        if count_3s == 1 {
            if count_2s == 1 {
                absolute_hand_values[i] = FULL_HOUSE;
            } else {
                absolute_hand_values[i] = THREE_OF_A_KIND;
            }
        } else if count_2s > 0 {
            if count_2s == 2 {
                absolute_hand_values[i] = TWO_PAIR;
            } else {
                absolute_hand_values[i] = PAIR;
            }
        } else {
            absolute_hand_values[i] = HIGH_CARD;
        }
    }
    // Pseudo-base conversion. Each card value occupies two digits of the number
    // The card values do not overlap each other's digits
    let mut hand_values: Vec<(usize, u32)> = Vec::with_capacity(hands.len());

    let base: u32 = 100;
    for i in 0..hands.len() {
        let mut num = 0;
        for j in 0..hands[0].len() {
            num += hands[i][j] * (100_000_000 / base.pow(i as u32));
        }
        hand_values.push((i, num));
    }
    hand_values.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", hand_values);
    // TODO: get all 5 of a kind hands and sort them by their pseudo-int
    
    // Assign relative card values, from high cards to 5 of a kind
    /*
    let mut ranking_to_assign = 1;
    let mut relative_hand_values: Vec<u32> = vec![0; hands.len()];
    let mut tied: Vec<usize> = vec![];
    for hand_val in 0..7 {
        tied.clear();
        for j in 0..absolute_hand_values.len() {
            if absolute_hand_values[j] == hand_val {
                tied.push(j);
            }
        }
        if tied.len() == 1 {
            relative_hand_values[tied[0]] = ranking_to_assign;
            ranking_to_assign += 1;
            continue;
        }
        if tied.len() < 2 {
            continue;
        }
        // Break any ties
        let rankings: Vec<u32> = break_tie(&mut tied, &hands);
        for hand_ind in rankings {
            relative_hand_values[hand_ind as usize] = ranking_to_assign;
            ranking_to_assign += 1;
        }
    }
    */

    // Note: relative ranks start at 1 for the weakest hand
    //return relative_hand_values;
    return absolute_hand_values;
}

fn break_tie(tied: &mut Vec<usize>, hands: &Vec<[u32; 5]>) -> Vec<u32> {
    // Break ties and return the newly-sorted indices
    let mut rankings: Vec<u32> = vec![0; tied.len()];
    // cumulative_values holds tuples of (hand index, partial sum of card values)
    let mut cumulative_values: Vec<(usize, u32)> = Vec::with_capacity(tied.len());
    for i in 0..tied.len() {
        cumulative_values.push((tied[i], 0));
    }
    let len_cum_vals = cumulative_values.len();
    let len_tied = tied.len();
    println!("len_cum_vals: {len_cum_vals}, len_tied: {len_tied}");

    let mut available_ranks: Vec<usize> = Vec::with_capacity(tied.len());
    for i in 0..tied.len() {
        available_ranks.push(i);
    }

    let mut card_in_hand_ind = 0;

    // Add card values to break ties. If a non-tied hand is found, give it a
    // ranking and remove it from the vectors used for comparing
    loop {
        assert_eq!(cumulative_values.len(), available_ranks.len());
        assert_eq!(tied.len(), available_ranks.len());
        if card_in_hand_ind > 4 {
            let l_cuml_vals = cumulative_values.len();
            let l_avail_ranks = available_ranks.len();
            let l_tied = tied.len();
            println!("l_c_r: {}, l_av_rk: {}, l_tied: {}", l_cuml_vals, l_avail_ranks, l_tied);
            panic!("card_in_hand_ind is {card_in_hand_ind}, should be <=4");
        }
        let orig_len_cml_vals = cumulative_values.len();
        let orig_len_avail_ranks = available_ranks.len();
        let orig_len_tied = tied.len();

        for i in 0..tied.len() {
            // breaks if changed to plain assignment
            cumulative_values[i].1 += hands[tied[i]][card_in_hand_ind];
        }
        cumulative_values.sort_by(|a, b| a.1.cmp(&b.1));
        println!("cuml_vals: {:?}", cumulative_values);

        let len_tied = tied.len();
        println!("len_tied: {len_tied}");
        // TODO: fix changing cumulative_vals while iterating over it
        for i in 0..cumulative_values.len() {
            if i >= cumulative_values.len() {
                break;
            }
            if cumulative_values.len() == 1
                || (i == 0 && cumulative_values[0].1 < cumulative_values[1].1)
            {
                rankings[available_ranks[0]] = cumulative_values[0].0 as u32;
                available_ranks.remove(0);
                let mut removed = false;
                for j in 0..tied.len() {
                    if tied[j] == cumulative_values[i].0 {
                        tied.remove(j);
                        removed = true;
                        break;
                    }
                }
                if !removed {
                    panic!("nothing removed");
                }
                cumulative_values.remove(i);
            } else if i == cumulative_values.len() - 1
                && cumulative_values[i - 1].1 < cumulative_values[i].1
            {
                rankings[available_ranks[i]] = cumulative_values[i].0 as u32;
                available_ranks.remove(i);
                let mut removed = false;
                let desired = cumulative_values[i].0;
                println!("trying to remove {desired}");
                for j in 0..tied.len() {
                    if tied[j] == cumulative_values[i].0 {
                        tied.remove(j);
                        removed = true;
                        break;
                    }
                }
                if !removed {
                    panic!("nothing removed");
                }
                cumulative_values.remove(i);
            } else if 0 < i
                && i < cumulative_values.len() - 1
                && cumulative_values[i - 1].1 < cumulative_values[i].1
                && cumulative_values[i].1 < cumulative_values[i + 1].1
            {
                let len_tied = tied.len();
                println!("len_tied: {len_tied}");
                rankings[available_ranks[i]] = cumulative_values[i].0 as u32;
                available_ranks.remove(i);
                let mut removed = false;
                for j in 0..tied.len() {
                    if tied[j] == cumulative_values[i].0 {
                        tied.remove(j);
                        removed = true;
                        break;
                    }
                }
                if !removed {
                    panic!("nothing removed");
                }
                cumulative_values.remove(i);
            }
        }
        if cumulative_values.len() == 0 && available_ranks.len() == 0 {
            break;
        }
        // In the next loop, add the value of the next card of each tied hand
        if orig_len_cml_vals == cumulative_values.len()
        && orig_len_avail_ranks == available_ranks.len()
        && orig_len_tied == tied.len() {
            card_in_hand_ind += 1;
        }
    }

    return rankings;
}
