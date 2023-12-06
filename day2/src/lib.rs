pub fn day_2_1(text: &str) -> u32 {
    // Return sum of possible game IDs if the bag contains 12 red,
    // 13 green, and 14 blue cubes
    let bag_contents = [12, 13, 14];
    let mut game_id_sum = 0;

    for line in text.lines() {
        let mut line = line.split(':');

        let game_id = line.next().unwrap().split(' ');
        let game_id: u32 = game_id.last().unwrap().parse::<u32>().unwrap();

        let mut is_game_valid = true;
        let game = line.next().unwrap().split(';');

        for round in game {
            if !is_game_valid {
                break;
            }
            for hand in round.split(',') {
                let mut rgb_max = [0, 0, 0];

                let mut tokens = hand.split(' ');
                tokens.next();
                let count: u32 = tokens.next().unwrap().parse::<u32>().unwrap();
                let color = tokens.next().unwrap();
                match color {
                    "red" => {
                        if count > rgb_max[0] {
                            rgb_max[0] = count;
                        }
                    }
                    "green" => {
                        if count > rgb_max[1] {
                            rgb_max[1] = count;
                        }
                    }
                    "blue" => {
                        if count > rgb_max[2] {
                            rgb_max[2] = count;
                        }
                    }
                    _ => panic!("invalid color: {color}"),
                }
                // If one round of a game is invalid, the game is invalid
                for i in 0..bag_contents.len() {
                    if rgb_max[i] > bag_contents[i] {
                        is_game_valid = false;
                        break;
                    }
                    is_game_valid = true;
                }
                if !is_game_valid {
                    break;
                }
            }
        }
        if is_game_valid {
            game_id_sum += game_id;
        }
    }
    return game_id_sum;
}
