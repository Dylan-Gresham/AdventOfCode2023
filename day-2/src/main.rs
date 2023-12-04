use std::fs;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn attempt_three() -> (u32, u32) {
    // Get the lines of the input file
    let file = fs::read_to_string("./input.txt").expect("File should exist: ");
    let lines: Vec<&str> = file.lines().collect::<Vec<&str>>();

    // Iterate through each line and sum together the valid game id's
    let mut sum_ids: u32 = 0;
    let mut sum_pows: u32 = 0;
    for line in lines {
        let game_data: Vec<&str> = line.split(": ").collect();
        // Get the game id
        let game_id = game_data[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let mut valid: bool = true; // Variable to keep track of whether a line is completely valid or not

        // Variables to keep track of the minimums of a line
        let mut red_cubes_values: Vec<u32> = vec!();
        let mut green_cubes_values: Vec<u32> = vec!();
        let mut blue_cubes_values: Vec<u32> = vec!();

        // Iterate through each round of drawing
        for draw in game_data[1].split("; ") {
            // For each color of cubes that were drawn in a round
            for cubes in draw.split(", ") {
                let split_cube: Vec<&str> = cubes.split_whitespace().collect(); // Get the number and color

                // Separate out into individual vars
                let num = split_cube[0].trim().parse::<u32>().unwrap();
                let color: &str = split_cube[1];
                
                // Check that no invalids were drawn and update mins if necessary
                match color {
                    "red" => {
                        if num > MAX_RED_CUBES {
                            valid = false;
                        }

                        red_cubes_values.push(num);
                    }
                    "green" => {
                        if num > MAX_GREEN_CUBES {
                            valid = false;
                        }

                        green_cubes_values.push(num);
                    }
                    "blue" => {
                        if num > MAX_BLUE_CUBES {
                            valid = false;
                        }
                        
                        blue_cubes_values.push(num);
                    }
                    _ => (),
                }
            }
        }

        // If all draws in the line were valid, add that id to the total and find the power of that round and add to that total too
        if valid {
            sum_ids += game_id;
        }

        sum_pows += red_cubes_values.iter().max().unwrap()
                        * green_cubes_values.iter().max().unwrap()
                        * blue_cubes_values.iter().max().unwrap();
    }

    (sum_ids, sum_pows)
}

fn main() {
    let (part1, part2) = attempt_three();
    println!("Total for Part 1: {}", part1);
    println!("Total for Part 2: {}", part2);
}
