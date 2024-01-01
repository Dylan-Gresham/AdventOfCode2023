use std::fs;

fn part_one() -> u32 {
    let file = fs::read_to_string("./input.txt").expect("File should exist");
    let lines = file.lines().collect::<Vec<&str>>();

    let times_raw: &str = lines[0].split(":").collect::<Vec<&str>>()[1].trim();
    let times: Vec<u32> = times_raw
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let distances_raw: &str = lines[1].split(":").collect::<Vec<&str>>()[1].trim();
    let distances: Vec<u32> = distances_raw
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut num_ways_to_win_per_race: Vec<u32> = vec![];
    for i in 0..times.len() {
        let goal_time = distances[i];
        let mut num_ways_to_win: u32 = 0;
        for time_holding in 0..times[i] {
            if time_holding * (times[i] - time_holding) > goal_time {
                num_ways_to_win += 1;
            }
        }
        num_ways_to_win_per_race.push(num_ways_to_win);
    }

    num_ways_to_win_per_race
        .iter()
        .fold(1, |cumprod, x| cumprod * x)
}

fn main() {
    println!("Total for part_one: {}", part_one());
}
