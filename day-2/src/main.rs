use std::fs;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn attempt_three() -> u32 {
    // Get the lines of the input file
    let file = fs::read_to_string("./input.txt").expect("File should exist: ");
    let lines: Vec<&str> = file.lines().collect::<Vec<&str>>();

    // Iterate through each line and sum together the valid game id's
    let mut sum: u32 = 0;
    for line in lines {
        let game_data: Vec<&str> = line.split(": ").collect();
        // Get the game id
        let game_id = game_data[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let mut valid: bool = true; // Variable to keep track of whether a line is completely valid or not

        // Iterate through each round of drawing
        for draw in game_data[1].split("; ") {
            // For each color of cubes that were drawn in a round
            for cubes in draw.split(", ") {
                let split_cube: Vec<&str> = cubes.split_whitespace().collect(); // Get the number and color

                // Separate out into individual vars
                let num = split_cube[0].trim().parse::<u32>().unwrap();
                let color: &str = split_cube[1];
                
                // Check that no invalids were drawn
                match color {
                    "red" => if num > MAX_RED_CUBES {valid = false;}
                    "green" => if num > MAX_GREEN_CUBES {valid = false; }
                    "blue" => if num > MAX_BLUE_CUBES {valid = false;}
                    _ => (),
                }

                if !valid {break;} // If there was an invalid exit early
            }

            if !valid {break;} // If there was an invalid exit early
        }

        // If all draws in the line were valid, add that id to the total
        if valid {
            sum += game_id;
        }
    }

    sum
}

fn main() {
    println!("Total for Part 1: {}", attempt_three());
}
