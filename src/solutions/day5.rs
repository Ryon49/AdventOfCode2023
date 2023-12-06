use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

impl Mapping {
    fn new(line: &str) -> Mapping {
        let map = line
            .split(" ")
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Mapping {
            destination: map[0],
            source: map[1],
            range: map[2],
        }
    }

    fn is_within_range(&self, source: u64) -> bool {
        return source >= self.source && source < self.source + self.range 
    }

    fn map(&self, source: u64) -> u64 {
        source - self.source + self.destination
    }
}

fn parse_input(
    _input: &str,
) -> (
    Vec<u64>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
) {
    let parts = _input.split("\n\n").collect::<Vec<&str>>();

    // parse seeds
    let seeds = parts[0]
        .replace("seeds: ", "")
        .split(" ")
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seed_to_soil = parts[1]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let soil_to_fertilizer = parts[2]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let fertilizer_to_water = parts[3]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let water_to_light = parts[4]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let light_to_temperature = parts[5]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let temperature_to_humidity = parts[6]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    let humidity_to_location = parts[7]
        .split("\n")
        .skip(1)
        .map(Mapping::new)
        .collect::<Vec<Mapping>>();

    return (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
}

fn calculate_convert(mappings: &Vec<Mapping>, source: u64) -> u64 {
    for mapping in mappings {
        if mapping.is_within_range(source) {
            return mapping.map(source)
        }
    }
    // no mapping found
    return source;
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u64 {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(_input);


    seeds.iter()
        .map(|seed| calculate_convert(&seed_to_soil, *seed))
        .map(|soil| calculate_convert(&soil_to_fertilizer, soil))
        .map(|fertilizer| calculate_convert(&fertilizer_to_water, fertilizer))
        .map(|water| calculate_convert(&water_to_light, water))
        .map(|light| calculate_convert(&light_to_temperature, light))
        .map(|temperature| calculate_convert(&temperature_to_humidity, temperature))
        .map(|humidity| calculate_convert(&humidity_to_location, humidity))
        .min().unwrap()
}

fn calculate_convert_with_range(mappings: &Vec<Mapping>, range: Range<u64>) {
    let result: Vec<Range<u64>> = Vec::new();

    

}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u64 {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(_input);

    // convert seeds to pairs
    seeds.chunks_exact(2)
        .map(|chunk| -> _ {
            let (start, range): (u64, u64) = (chunk[0], chunk[1]);
            start..start + range
        })
        .flatten()
        .map(|seed| calculate_convert(&seed_to_soil, seed))
        .map(|soil| calculate_convert(&soil_to_fertilizer, soil))
        .map(|fertilizer| calculate_convert(&fertilizer_to_water, fertilizer))
        .map(|water| calculate_convert(&water_to_light, water))
        .map(|light| calculate_convert(&light_to_temperature, light))
        .map(|temperature| calculate_convert(&temperature_to_humidity, temperature))
        .map(|humidity| calculate_convert(&humidity_to_location, humidity))
        .min().unwrap()

}