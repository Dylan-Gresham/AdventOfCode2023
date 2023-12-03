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
    Lines::sum_all_valids(Lines::from_file("./test-input.txt"))
}

fn main() {
    println!("Total for Part 1: {}", part_one());
}
