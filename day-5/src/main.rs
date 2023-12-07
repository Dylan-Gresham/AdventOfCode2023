use std::fs;

#[derive(Debug)]
struct SeedSoil {
    seed_range: Vec<u32>,
    soil_range: Vec<u32>,
}

impl SeedSoil {
    fn new() -> Self {
        Self {
            seed_range: vec![],
            soil_range: vec![],
        }
    }

    fn add_pair(&mut self, seed: &u32, soil: &u32) {
        self.seed_range.push(*seed);
        self.soil_range.push(*soil);
    }
}

#[derive(Debug)]
struct SoilFert {
    soil_range: Vec<u32>,
    fert_range: Vec<u32>,
}

impl SoilFert {
    fn new() -> Self {
        Self {
            soil_range: vec![],
            fert_range: vec![],
        }
    }

    fn add_pair(&mut self, soil: &u32, fert: &u32) {
        self.soil_range.push(*soil);
        self.fert_range.push(*fert);
    }
}

#[derive(Debug)]
struct FertWater {
    fert_range: Vec<u32>,
    water_range: Vec<u32>,
}

impl FertWater {
    fn new() -> Self {
        Self {
            fert_range: vec![],
            water_range: vec![],
        }
    }

    fn add_pair(&mut self, fert: &u32, water: &u32) {
        self.fert_range.push(*fert);
        self.water_range.push(*water);
    }
}

#[derive(Debug)]
struct WaterLight {
    water_range: Vec<u32>,
    light_range: Vec<u32>,
}

impl WaterLight {
    fn new() -> Self {
        Self {
            water_range: vec![],
            light_range: vec![],
        }
    }

    fn add_pair(&mut self, water: &u32, light: &u32) {
        self.water_range.push(*water);
        self.light_range.push(*light);
    }
}

#[derive(Debug)]
struct LightTemp {
    light_range: Vec<u32>,
    temp_range: Vec<u32>,
}

impl LightTemp {
    fn new() -> Self {
        Self {
            light_range: vec![],
            temp_range: vec![],
        }
    }

    fn add_pair(&mut self, light: &u32, temp: &u32) {
        self.light_range.push(*light);
        self.temp_range.push(*temp);
    }
}

#[derive(Debug)]
struct TempHumid {
    temp_range: Vec<u32>,
    humid_range: Vec<u32>,
}

impl TempHumid {
    fn new() -> Self {
        Self {
            temp_range: vec![],
            humid_range: vec![],
        }
    }

    fn add_pair(&mut self, temp: &u32, humid: &u32) {
        self.temp_range.push(*temp);
        self.humid_range.push(*humid);
    }
}

#[derive(Debug)]
struct HumidLoc {
    humid_range: Vec<u32>,
    loc_range: Vec<u32>,
}

impl HumidLoc {
    fn new() -> Self {
        Self {
            humid_range: vec![],
            loc_range: vec![],
        }
    }

    fn add_pair(&mut self, humid: &u32, loc: &u32) {
        self.humid_range.push(*humid);
        self.loc_range.push(*loc);
    }
}

fn part_one() -> u32 {
    let mut location_vals: Vec<u32> = vec![];
    let file = fs::read_to_string("./test-input.txt")
        .expect("File should exist at file path relative to crate/src");
    let lines: Vec<&str> = file
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut seeds: Vec<u32> = vec![];
    let mut seed_to_soil = SeedSoil::new();
    let mut soil_to_fert = SoilFert::new();
    let mut fert_to_water = FertWater::new();
    let mut water_to_light = WaterLight::new();
    let mut light_to_temp = LightTemp::new();
    let mut temp_to_humid = TempHumid::new();
    let mut humid_to_location = HumidLoc::new();

    let mut curr_tag: &str = "";
    let mut change_tag: bool = false;
    for mut line in lines {
        change_tag = false;
        line = line.trim();

        if line.contains("seeds: ") {
            curr_tag = "seeds";
            change_tag = true;
            let colon_idx = line.find(":").unwrap();
            seeds = line[colon_idx + 1..]
                .split_whitespace()
                .map(|val| val.parse::<u32>().unwrap())
                .collect();
        } else {
            match line.trim() {
                "seed-to-soil map:" => {
                    curr_tag = "sts";
                    change_tag = true;
                }
                "soil-to-fertilizer map:" => {
                    curr_tag = "stf";
                    change_tag = true;
                }
                "fertilizer-to-water map:" => {
                    curr_tag = "ftw";
                    change_tag = true;
                }
                "water-to-light map:" => {
                    curr_tag = "wtl";
                    change_tag = true;
                }
                "light-to-temperature map:" => {
                    curr_tag = "ltt";
                    change_tag = true;
                }
                "temperature-to-humidity map:" => {
                    curr_tag = "tth";
                    change_tag = true;
                }
                "humidity-to-location map:" => {
                    curr_tag = "htl";
                    change_tag = true;
                }
                _ => (),
            }
        }

        if !change_tag && curr_tag != "seeds" {
            let line_split: Vec<u32> = line
                .split_whitespace()
                .map(|val| val.trim().parse::<u32>().unwrap())
                .collect();

            let step_val = line_split[2];

            let mut first_values = {
                let mut ret: Vec<u32> = vec![];
                for increment_value in 0..step_val {
                    ret.push(line_split[0] + increment_value);
                }
                ret
            };
            println!();

            let mut second_values = {
                let mut ret: Vec<u32> = vec![];
                for increment_value in 0..step_val {
                    ret.push(line_split[1] + increment_value);
                }
                ret
            };

            match curr_tag {
                "sts" => {
                    while !first_values.is_empty() {
                        seed_to_soil
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "stf" => {
                    while !first_values.is_empty() {
                        soil_to_fert
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "ftw" => {
                    while !first_values.is_empty() {
                        fert_to_water
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "wtl" => {
                    while !first_values.is_empty() {
                        water_to_light
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "ltt" => {
                    while !first_values.is_empty() {
                        light_to_temp
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "tth" => {
                    while !first_values.is_empty() {
                        temp_to_humid
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                "htl" => {
                    while !first_values.is_empty() {
                        humid_to_location
                            .add_pair(&first_values.pop().unwrap(), &second_values.pop().unwrap());
                    }
                }
                _ => (),
            }
        }
    }

    println!("{:?}", seeds);
    println!("{:?}", seed_to_soil);
    println!("{:?}", soil_to_fert);
    println!("{:?}", fert_to_water);
    println!("{:?}", water_to_light);
    println!("{:?}", light_to_temp);
    println!("{:?}", temp_to_humid);
    println!("{:?}", humid_to_location);

    for seed in seeds {
        let mut next_val: u32 = seed;
        if seed_to_soil.seed_range.contains(&next_val) {
            next_val = seed_to_soil.soil_range[seed_to_soil
                .seed_range
                .iter()
                .position(|s| *s == next_val)
                .unwrap()];
        }

        if soil_to_fert.soil_range.contains(&next_val) {
            next_val = soil_to_fert.fert_range[soil_to_fert
                .soil_range
                .iter()
                .position(|s| *s == next_val)
                .unwrap()];
        }

        if fert_to_water.fert_range.contains(&next_val) {
            next_val = fert_to_water.water_range[fert_to_water
                .fert_range
                .iter()
                .position(|f| *f == next_val)
                .unwrap()];
        }

        if water_to_light.water_range.contains(&next_val) {
            next_val = water_to_light.light_range[water_to_light
                .water_range
                .iter()
                .position(|w| *w == next_val)
                .unwrap()];
        }

        if light_to_temp.light_range.contains(&next_val) {
            next_val = light_to_temp.temp_range[light_to_temp
                .light_range
                .iter()
                .position(|l| *l == next_val)
                .unwrap()];
        }

        if temp_to_humid.temp_range.contains(&next_val) {
            next_val = temp_to_humid.humid_range[temp_to_humid
                .temp_range
                .iter()
                .position(|t| *t == next_val)
                .unwrap()];
        }

        if humid_to_location.humid_range.contains(&next_val) {
            next_val = humid_to_location.loc_range[humid_to_location
                .humid_range
                .iter()
                .position(|h| *h == next_val)
                .unwrap()];
        }

        location_vals.push(next_val);
    }

    println!("{:?}", location_vals);
    *location_vals.iter().min().unwrap()
}

fn main() {
    println!("Total for part one: {}", part_one());
}
