pub fn day_6_1(text: &str) -> u64 {
    let (mut race_durations, mut best_distances) = text.split_once('\n').unwrap();

    (_, race_durations) = race_durations.split_once(':').unwrap();
    let race_durations = race_durations
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap_or_default())
        .collect::<Vec<u64>>();

    (_, best_distances) = best_distances.split_once(':').unwrap();
    let best_distances = best_distances
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap_or_default())
        .collect::<Vec<u64>>();
    let mut ways_to_win_per_game = vec![0; race_durations.len()];

    for race in 0..race_durations.len() {
        let mut ways_to_win = 0;
        for wait_time in 0..race_durations[race] {
            let velocity = wait_time;
            let distance_travelled = velocity * (race_durations[race] - wait_time);
            if distance_travelled > best_distances[race] {
                ways_to_win += 1;
            }
        }
        ways_to_win_per_game[race] = ways_to_win;
    }

    return ways_to_win_per_game.iter().fold(1, |acc, b| acc * b);
}

pub fn day_6_2(text: &str) -> u64 {
    let (mut race_duration, mut best_distance) = text.split_once('\n').unwrap();

    (_, race_duration) = race_duration.split_once(':').unwrap();
    let race_duration: u64 = race_duration
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap_or_default();

    (_, best_distance) = best_distance.split_once(':').unwrap();
    let best_distance = best_distance
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap_or_default();

    let mut ways_to_win = 0;
    let mut velocity;
    let mut distance;
    let min_avg_velocity_needed = best_distance / race_duration;

    for wait_time in min_avg_velocity_needed..race_duration {
        velocity = wait_time;
        distance = velocity * (race_duration - wait_time);
        if distance > best_distance {
            ways_to_win += 1;
        }
    }

    return ways_to_win;
}
