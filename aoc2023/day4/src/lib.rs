pub fn day_4_1(text: &str) -> i32 {
    let mut total_value = 0;
    for line in text.lines() {
        let (_, line) = line.split_once(':').unwrap();
        let (winning_nums, my_nums) = line.split_once('|').unwrap();

        // TODO: the next two blocks do the same operations, try to condense them.
        // TODO: maybe  try collect() to build the vec?
        let mut temp = Vec::new();
        for num in winning_nums.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            temp.push(num.parse::<i32>().unwrap());
        }
        let winning_nums = temp;

        let mut temp = Vec::new();
        for num in my_nums.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            temp.push(num.parse::<i32>().unwrap());
        }
        let my_nums = temp;

        let mut card_value = 0;
        for win_n in winning_nums.iter() {
            if my_nums.contains(win_n) {
                if card_value == 0 {
                    card_value += 1;
                } else {
                    card_value *= 2;
                }
            }
        }
        total_value += card_value;
    }

    return total_value;
}

pub fn day_4_2(text: &str) -> i32 {
    let mut owned_cards: Vec<i32> = Vec::new();
    let mut card_values: Vec<usize> = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let (_, line) = line.split_once(':').unwrap();
        let (winning_nums, my_nums) = line.split_once('|').unwrap();

        // TODO: the next two blocks do the same operations, try to condense them.
        // TODO: maybe  try collect() to build the vec?
        let mut temp = Vec::new();
        for num in winning_nums.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            temp.push(num.parse::<i32>().unwrap());
        }
        let winning_nums = temp;

        let mut temp = Vec::new();
        for num in my_nums.trim().split(' ') {
            if num.is_empty() {
                continue;
            }
            temp.push(num.parse::<i32>().unwrap());
        }
        let my_nums = temp;

        let mut card_value = 0;
        for win_n in winning_nums.iter() {
            if my_nums.contains(win_n) {
                card_value += 1;
            }
        }
        card_values.push(card_value);

        while i + card_value >= owned_cards.len() {
            owned_cards.push(0);
        }
        // Add this card
        owned_cards[i] += 1;
    }

    // Calculate amount of card copies
    for i in 0..card_values.len() {
        if owned_cards[i] > 0 && card_values[i] > 0 {
            for j in 1..card_values[i] + 1 {
                owned_cards[i + j] += owned_cards[i];
            }
        }
    }

    return owned_cards.iter().sum();
}
