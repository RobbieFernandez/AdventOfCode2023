use itertools::Itertools;
use std::{collections::HashMap, env, fs::read_to_string, ops::Range};

#[derive(Debug)]
struct RangeMap {
    source_start: u32,
    dest_start: u32,
    length: u32,
}

#[derive(Debug)]
struct Map {
    range_maps: Vec<RangeMap>,
}

#[derive(Debug)]
struct CategoryMaps {
    // { source: { dest: { int: int }}}
    category_map: HashMap<String, HashMap<String, Map>>,
}

impl RangeMap {
    fn new(dest_start: u32, source_start: u32, length: u32) -> Self {
        Self {
            source_start,
            dest_start,
            length,
        }
    }

    fn source_in_range(&self, val: u32) -> bool {
        val >= self.source_start && val - self.source_start < self.length
    }

    fn dest_in_range(&self, val: u32) -> bool {
        val >= self.dest_start && val - self.dest_start < self.length
    }

    fn map_value(&self, source: u32) -> u32 {
        let offset = source - self.source_start;
        self.dest_start + offset
    }

    fn reverse_map_value(&self, dest: u32) -> u32 {
        let offset = dest - self.dest_start;
        self.source_start + offset
    }
}

impl Map {
    fn new() -> Self {
        let range_maps: Vec<RangeMap> = Vec::new();
        Self { range_maps }
    }

    fn add_range(&mut self, dest_start: u32, source_start: u32, length: u32) {
        let range = RangeMap::new(dest_start, source_start, length);
        self.range_maps.push(range);
    }

    fn get(&self, key: u32) -> u32 {
        self.range_maps
            .iter()
            .filter(|r| r.source_in_range(key))
            .map(|r| r.map_value(key))
            .next()
            .unwrap_or(key)
    }

    fn get_reverse(&self, key: u32) -> u32 {
        self.range_maps
            .iter()
            .filter(|r| r.dest_in_range(key))
            .map(|r| r.reverse_map_value(key))
            .next()
            .unwrap_or(key)
    }
}

impl CategoryMaps {
    fn new() -> Self {
        let category_map: HashMap<String, HashMap<String, Map>> = HashMap::new();
        Self { category_map }
    }

    fn get_map(&self, from_category: &str, to_category: &str) -> &Map {
        self.category_map
            .get(&from_category.to_string())
            .expect("source category should be mapped")
            .get(&to_category.to_string())
            .expect("destination category should be mapped")
    }

    fn add_map(&mut self, from_category: &str, to_category: &str, map: Map) {
        let from_category = from_category.to_string();
        if !self.category_map.contains_key(&from_category) {
            self.category_map
                .insert(from_category.clone(), HashMap::new());
        }

        let outer_map = self.category_map.get_mut(&from_category).unwrap();
        outer_map.insert(to_category.to_string(), map);
    }

    fn get_location(&self, layer_name: &str, value: u32) -> u32 {
        match layer_name {
            "seed" => self.seed_to_location(value),
            "soil" => self.soil_to_location(value),
            "fertilizer" => self.fertilizer_to_location(value),
            "water" => self.water_to_location(value),
            "light" => self.light_to_location(value),
            "temperature" => self.temperature_to_location(value),
            "humidity" => self.humidity_to_location(value),
            _ => panic!("Unknown layer: {}", layer_name),
        }
    }

    fn get_seed(&self, layer_name: &str, value: u32) -> u32 {
        match layer_name {
            "soil" => self.soil_to_seed(value),
            "fertilizer" => self.fertilizer_to_seed(value),
            "water" => self.water_to_seed(value),
            "light" => self.light_to_seed(value),
            "temperature" => self.temperature_to_seed(value),
            "humidity" => self.humidity_to_seed(value),
            "location" => self.location_to_seed(value),
            _ => panic!("Unknown layer: {}", layer_name),
        }
    }

    fn seed_to_location(&self, seed: u32) -> u32 {
        // seed -> soil
        let seed_to_soil = self.get_map("seed", "soil");
        let soil = seed_to_soil.get(seed);
        self.soil_to_location(soil)
    }

    fn soil_to_location(&self, soil: u32) -> u32 {
        let soil_to_fertilizer = self.get_map("soil", "fertilizer");
        let fertilizer = soil_to_fertilizer.get(soil);
        self.fertilizer_to_location(fertilizer)
    }

    fn fertilizer_to_location(&self, fertilizer: u32) -> u32 {
        // fertilizer -> water
        let fertilizer_to_water = self.get_map("fertilizer", "water");
        let water = fertilizer_to_water.get(fertilizer);
        self.water_to_location(water)
    }

    fn water_to_location(&self, water: u32) -> u32 {
        let water_to_light = self.get_map("water", "light");
        let light = water_to_light.get(water);
        self.light_to_location(light)
    }

    fn light_to_location(&self, light: u32) -> u32 {
        let light_to_temperature = self.get_map("light", "temperature");
        let temperature = light_to_temperature.get(light);
        self.temperature_to_location(temperature)
    }

    fn temperature_to_location(&self, temperature: u32) -> u32 {
        let temperature_to_humidity = self.get_map("temperature", "humidity");
        let humidity = temperature_to_humidity.get(temperature);
        self.humidity_to_location(humidity)
    }

    fn humidity_to_location(&self, humidity: u32) -> u32 {
        let humidity_to_location = self.get_map("humidity", "location");
        humidity_to_location.get(humidity)
    }

    // Reverse mapping
    fn soil_to_seed(&self, soil: u32) -> u32 {
        let seed_to_soil = self.get_map("seed", "soil");
        seed_to_soil.get_reverse(soil)
    }

    fn fertilizer_to_seed(&self, ferilizer: u32) -> u32 {
        let soil_to_fertilizer = self.get_map("soil", "fertilizer");
        let soil = soil_to_fertilizer.get_reverse(ferilizer);
        self.soil_to_seed(soil)
    }

    fn water_to_seed(&self, water: u32) -> u32 {
        let fertilizer_to_water = self.get_map("fertilizer", "water");
        let fertilizer = fertilizer_to_water.get_reverse(water);
        self.fertilizer_to_seed(fertilizer)
    }

    fn light_to_seed(&self, light: u32) -> u32 {
        let water_to_light = self.get_map("water", "light");
        let water = water_to_light.get_reverse(light);
        self.water_to_seed(water)
    }

    fn temperature_to_seed(&self, temperature: u32) -> u32 {
        let light_to_temperature = self.get_map("light", "temperature");
        let light = light_to_temperature.get_reverse(temperature);
        self.light_to_seed(light)
    }

    fn humidity_to_seed(&self, humidity: u32) -> u32 {
        let temperature_to_humidity = self.get_map("temperature", "humidity");
        let temperature = temperature_to_humidity.get_reverse(humidity);
        self.temperature_to_seed(temperature)
    }

    fn location_to_seed(&self, location: u32) -> u32 {
        let humidity_to_location = self.get_map("humidity", "location");
        let humidity = humidity_to_location.get_reverse(location);
        self.humidity_to_seed(humidity)
    }
}

fn parse_input(filename: &str) -> (Vec<u32>, CategoryMaps) {
    let input = read_to_string(filename).expect("file should exist");
    let mut lines = input.lines();

    // First we get seed numbers.
    let seeds = lines.next().unwrap();
    let seeds = seeds.split(":").nth(1).unwrap();
    let seeds = seeds
        .trim()
        .split(' ')
        .map(|n| n.trim().parse::<u32>().unwrap());

    // Throw away the leading blank line
    lines.next();

    // Now go in chunks delimited by blank lines
    let grouped = lines.group_by(|l| l.trim() == "");
    let grouped = grouped.into_iter();
    let grouped = grouped.filter(|(is_blank, _)| !is_blank);

    let mut category_maps = CategoryMaps::new();

    for (_, mut line_chunk) in grouped {
        // let mut line_chunk = line_chunk.filter(|l| l != &"");

        // First line contains source/dest category name
        let header = line_chunk.next().unwrap();
        let header = header.split(' ').next().unwrap();
        let mut header = header.split("-to-");
        let source_category = header.next().unwrap();
        let dest_category = header.next().unwrap();

        let mut map = Map::new();

        for line in line_chunk {
            let mut numbers = line
                .split(' ')
                .map(|c| c.parse::<u32>().expect("Invalid number"));

            let dest_range_start = numbers.next().unwrap();
            let source_range_start = numbers.next().unwrap();
            let range_length = numbers.next().unwrap();
            map.add_range(dest_range_start, source_range_start, range_length)
        }

        category_maps.add_map(source_category, dest_category, map)
    }

    (seeds.collect(), category_maps)
}

fn solution1(seeds: &[u32], category_maps: &CategoryMaps) -> u32 {
    seeds
        .iter()
        .map(|seed| category_maps.seed_to_location(*seed))
        .min()
        .unwrap()
}

fn solution2(seeds: &[u32], category_maps: &CategoryMaps) -> u32 {
    // Convert list of seeds to ranges of seeds
    let seed_ranges: Vec<Range<u32>> = seeds
        .chunks(2)
        .map(|seed_pair| {
            let start = seed_pair[0];
            let length = seed_pair[1];

            Range {
                start,
                end: start + length,
            }
        })
        .collect();

    // First look at the start of each seed range,
    let first_minimum = seed_ranges
        .iter()
        .map(|r| category_maps.seed_to_location(r.start))
        .min()
        .unwrap();

    // Go through each layer of the map, taking the bottom of each range.
    let layers = vec![
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location"),
    ];

    let min_from_layers = layers
        .iter()
        .map(|(from, to)| {
            let map = category_maps.get_map(from, to);
            map.range_maps
                .iter()
                .filter(|r| {
                    let seed = category_maps.get_seed(from, r.source_start);
                    seed_ranges
                        .iter()
                        .any(|seed_range| seed_range.contains(&seed))
                })
                .map(|r| category_maps.get_location(from, r.source_start))
                .min()
                .unwrap_or(u32::MAX)
        })
        .min()
        .unwrap();

    std::cmp::min(first_minimum, min_from_layers)
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");
    let (seeds, category_maps) = parse_input(&filename);

    let answer1 = solution1(&seeds, &category_maps);
    println!("{}", answer1);

    let answer2 = solution2(&seeds, &category_maps);
    println!("{}", answer2);
}
