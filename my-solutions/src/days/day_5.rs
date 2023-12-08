use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};
use rudvent_lib::solution::{Solution, SolutionBuilder, StructSolutionBuilder};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Almanac;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 35;
const EXAMPLE_2_ANS: OutputPart2 = 46;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

struct RangeConverter {
    start: usize,
    end: usize,
    offset: isize,
}

impl RangeConverter {
    fn convert(&self, source: usize) -> Option<usize> {
        if source < self.start || source > self.end {
            None
        } else {
            Some(source.checked_add_signed(self.offset).unwrap())
        }
    }
}

struct Map<T, V> {
    converters: Vec<RangeConverter>,
    _phantom: PhantomData<(T, V)>,
}

impl<T: NewType + Eq + Hash, V: NewType> Map<T, V> {
    fn from_vec(vec: &Vec<(usize, usize, usize)>) -> Map<T, V> {
        let mut converters = Vec::new();
        for (dest_start, source_start, len) in vec {
            let converter = RangeConverter {
                start: *source_start,
                end: source_start + len,
                offset: (*dest_start as isize) - (*source_start as isize),
            };
            converters.push(converter)
        }
        Map {
            converters,
            _phantom: Default::default(),
        }
    }

    fn get(&self, source: T) -> V {
        for con in &self.converters {
            if let Some(res) = con.convert(source.get()) {
                return V::from_usize(res);
            }
        }
        V::from_usize(source.get())
    }
}

trait NewType {
    fn from_usize(u: usize) -> Self;
    fn get(&self) -> usize;
}

macro_rules! newtype {
    ($e:ident) => {
        #[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
        struct $e(usize);

        impl NewType for $e {
            fn from_usize(u: usize) -> $e {
                $e(u)
            }
            fn get(&self) -> usize {
                self.0
            }
        }
    }
}

// This instantiates a load of type-safe NewTypes that derive the macros and can be converted
// to and from string. This way it should be impossible to mix up a temperature and a light
// for example, despite them both being internally represented as strings
newtype!(Seed);
newtype!(Soil);
newtype!(Fertiliser);
newtype!(Water);
newtype!(Light);
newtype!(Temperature);
newtype!(Humidity);
newtype!(Location);

pub struct Almanac {
    seeds: Vec<Seed>,
    seed_soil: Map<Seed, Soil>,
    soil_fert: Map<Soil, Fertiliser>,
    fert_water: Map<Fertiliser, Water>,
    water_light: Map<Water, Light>,
    light_temp: Map<Light, Temperature>,
    temp_hum: Map<Temperature, Humidity>,
    hum_loc: Map<Humidity, Location>,
}

impl Almanac {
    fn seed_location(&self, seed: &Seed) -> Location {
        self.hum_loc.get(
            self.temp_hum.get(
                self.light_temp.get(
                    self.water_light.get(
                        self.fert_water
                            .get(self.soil_fert.get(self.seed_soil.get(*seed))),
                    ),
                ),
            ),
        )
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac_blocks = s.split("\n\n");
        let seed_line = almanac_blocks.next().unwrap().trim();
        let seeds = seed_line
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|seed| Seed::from_usize(usize::from_str(seed).unwrap()))
            .collect();

        let string_blocks = almanac_blocks
            .map(|block| {
                let block = block.trim();
                let mut lines = block.lines();
                (
                    lines.next().unwrap(),
                    lines
                        .map(|line| {
                            let mut three_elements = line.split_whitespace();
                            (
                                usize::from_str(three_elements.next().unwrap()).unwrap(),
                                usize::from_str(three_elements.next().unwrap()).unwrap(),
                                usize::from_str(three_elements.next().unwrap()).unwrap(),
                            )
                        })
                        .collect(),
                )
            })
            .collect::<HashMap<&str, Vec<(usize, usize, usize)>>>();
        let seed_soil: Map<Seed, Soil> =
            Map::from_vec(string_blocks.get("seed-to-soil map:").unwrap());
        let soil_fert: Map<Soil, Fertiliser> =
            Map::from_vec(string_blocks.get("soil-to-fertilizer map:").unwrap());
        let fert_water: Map<Fertiliser, Water> =
            Map::from_vec(string_blocks.get("fertilizer-to-water map:").unwrap());
        let water_light: Map<Water, Light> =
            Map::from_vec(string_blocks.get("water-to-light map:").unwrap());
        let light_temp: Map<Light, Temperature> =
            Map::from_vec(string_blocks.get("light-to-temperature map:").unwrap());
        let temp_hum: Map<Temperature, Humidity> =
            Map::from_vec(string_blocks.get("temperature-to-humidity map:").unwrap());
        let hum_loc: Map<Humidity, Location> =
            Map::from_vec(string_blocks.get("humidity-to-location map:").unwrap());
        Ok(Almanac {
            seeds,
            seed_soil,
            soil_fert,
            fert_water,
            water_light,
            light_temp,
            temp_hum,
            hum_loc,
        })
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Almanac::from_str(&input).unwrap()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<UserMonitor>,
) -> OutputPart1 {
    input
        .seeds
        .iter()
        .map(|seed| input.seed_location(seed))
        .min()
        .unwrap()
        .get()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<UserMonitor>,
) -> OutputPart1 {
    input
        .seeds
        .iter()
        .fold(
            (None, Vec::<(Seed, usize)>::new()),
            |(prev_seed, mut vec), seed| match prev_seed {
                None => (Some(seed), vec),
                Some(prev) => {
                    vec.push((*prev, seed.get()));
                    (None, vec)
                }
            },
        )
        .1
        .iter()
        .flat_map(|(start_seed, len)| {
            (start_seed.get()..=start_seed.get() + len).map(|seed| input.seed_location(&Seed(seed)))
        })
        .min()
        .unwrap()
        .get()
}

// ----- There is no need to change anything below this line -----
// The below code creates a solution that is generic over several types. These types might change
// between different days, for example integers on some and strings on others. They are type-aliased
// above to make it easier to change them all at once
pub fn make_sol() -> Box<dyn SolutionBuilder> {
    let sol = StructSolutionBuilder::new(
        prepare,
        part_1,
        prepare_2,
        part_2,
        Example::Value(EXAMPLE_1_ANS),
        Example::Value(EXAMPLE_2_ANS),
    );
    Box::new(sol)
}
