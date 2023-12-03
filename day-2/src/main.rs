use std::fs;

const MAX_RED_CUBES: u32 = 12;
const MAX_BLUE_CUBES: u32 = 14;
const MAX_GREEN_CUBES: u32 = 13;

struct Lines {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Lines {
    fn new(id: u32, red: u32, green: u32, blue: u32) -> Self {
        Self {
            id,
            red,
            green,
            blue,
        }
    }

    fn from_file(file_path: &str) -> Vec<Self> {
        fs::read_to_string(file_path)
            .expect("File should exist but didn't")
            .lines()
            .map(|line| {
                let id: u32 = {
                    let mut id_chars = String::new();
                    for c in line.chars() {
                        if c.is_numeric() {
                            id_chars.push(c);
                        } else if c == ':' {
                            break;
                        }
                    }
                    id_chars.parse::<u32>().unwrap()
                };

                let mut red: u32 = 0;
                let mut green: u32 = 0;
                let mut blue: u32 = 0;

                let past_colon = &line[line.find(':').unwrap() + 2..].trim();
                let between_semicolons = past_colon.split(";").collect::<Vec<&str>>();
                for part in between_semicolons {
                    let comma_split = part.split(",").collect::<Vec<&str>>();
                    for combo in comma_split {
                        let combo_pair = combo.split_whitespace().collect::<Vec<&str>>();
                        let num = combo_pair[0].trim().parse::<u32>().unwrap();
                        let color = combo_pair[1].trim();

                        let _ = match color {
                            "red" => red += num,
                            "green" => green += num,
                            "blue" => blue += num,
                            _ => (),
                        };
                    }
                }

                Self::new(id, red, green, blue)
            })
            .collect::<Vec<Lines>>()
    }

    fn is_valid(&self) -> bool {
        if self.red <= MAX_RED_CUBES && self.green <= MAX_GREEN_CUBES && self.blue <= MAX_BLUE_CUBES
        {
            true
        } else {
            false
        }
    }

    fn sum_all_valids(lines: Vec<Self>) -> u32 {
        let mut sum: u32 = 0;

        for line in lines {
            if line.is_valid() {
                sum += line.id;
            }
        }

        sum
    }
}

fn part_one() -> u32 {
    //    Lines::sum_all_valids(Lines::from_file("./input.txt"))
    let valid_ids: Vec<u32> = {
        let binding = fs::read_to_string("./input.txt").unwrap();

        let lines = binding
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();

        let mut id_vec: Vec<u32> = vec![];

        for line in lines {
            let index_of_colon = line.find(':').unwrap();
            let mut num_reds: u32 = 0;
            let mut num_greens: u32 = 0;
            let mut num_blues: u32 = 0;
            let id: u32 = line[5..index_of_colon].parse().unwrap();
            let data_section: Vec<&str> = line[index_of_colon + 2..].split(";").collect();
            for data in data_section {
                let num_color_pairs: Vec<&str> = data.split(", ").collect();
                for pair in num_color_pairs {
                    let values: Vec<&str> = pair.split_whitespace().collect();
                    let num: u32 = values[0].parse().unwrap();
                    let color = values[1];
                    match color {
                        "red" => num_reds += num,
                        "green" => num_greens += num,
                        "blue" => num_blues += num,
                        _ => (),
                    }
                }
            }

            if num_reds <= MAX_RED_CUBES
                && num_greens <= MAX_GREEN_CUBES
                && num_blues <= MAX_BLUE_CUBES
            {
                id_vec.push(id);
            }
        }
        id_vec
    };
    valid_ids.into_iter().sum()
}

fn main() {
    println!("Total for Part 1: {}", part_one());
}
