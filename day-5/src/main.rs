use std::fs;

fn part_one() -> u32 {
    // Get the lines of the file
    let file = fs::read_to_string("./input.txt").expect("File should exist");
    let mut lines = file.lines().collect::<Vec<&str>>();
    lines.reverse();

    // Get all the seeds
    let seeds = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Get all the mappings
    let mut maps: Vec<Vec<(u32, u32, u32)>> = vec![];
    let mut set: Vec<(u32, u32, u32)> = vec![];
    while let Some(mut line) = lines.pop() {
        if line.is_empty() || line == "" {
            continue;
        } else if line.contains("-") {
            maps.push(set);
            set = vec![];
        } else {
            while !line.is_empty() {
                let parts: Vec<u32> = line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                let (dst_start, src_start, range) = (parts[0], parts[1], parts[2]);
                set.push((dst_start, src_start, range));
                if let Some(next_line) = lines.pop() {
                    line = next_line;
                } else {
                    break;
                }
            }
        }
    }

    fn find_loc(seed: u32, maps: &Vec<Vec<(u32, u32, u32)>>) -> u32 {
        let mut curr_num = seed.clone();

        for map in maps.iter() {
            for &(dst_start, src_start, range) in map.iter() {
                let max: u64 = u64::from(src_start) + u64::from(range);
                if src_start <= curr_num && u64::from(curr_num) < max {
                    curr_num = dst_start + (curr_num - src_start);
                    break;
                }
            }
        }

        curr_num
    }

    let mut locations: Vec<u32> = vec![];
    for seed in seeds.iter() {
        let loc = find_loc(*seed, &maps);
        locations.push(loc);
    }

    if let Some(min_location) = locations.iter().min() {
        *min_location
    } else {
        0
    }
}

fn main() {
    println!("Total for part one: {}", part_one());
}
