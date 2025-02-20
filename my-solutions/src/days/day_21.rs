use std::fmt::Display;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Vec<char>>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 126384;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(|s| s.chars().collect()).collect()
}

const KEYS: usize = 11;
const DIRS: usize = 5;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Keypress {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Keypress {
    fn towards(start: usize, target: usize, horizontal: bool) -> Option<(usize, Keypress)> {
        if start == target {
            return None
        } else if start < target {
            if horizontal {
                return Some((start + 1, Keypress::Right))
            } else {
                return Some((start + 1, Keypress::Down))
            }
        } else {
            if horizontal {
                return Some((start - 1, Keypress::Left))
            } else {
                return Some((start - 1, Keypress::Up))
            }
        }
    }
}

impl From<usize> for Keypress {
    fn from(value: usize) -> Self {
        match value {
            0 => Keypress::Up,
            1 => Keypress::Down,
            2 => Keypress::Left,
            3 => Keypress::Right,
            4 => Keypress::Activate,
            _ => panic!("Int not recognised")
        }
    }
}

impl From<Keypress> for usize {
    fn from(value: Keypress) -> Self {
        match value {
            Keypress::Up => 0,
            Keypress::Down => 1,
            Keypress::Left => 2,
            Keypress::Right => 3,
            Keypress::Activate => 4,
        }
    }
}

impl Display for Keypress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Keypress::Up => "^",
            Keypress::Down => "v",
            Keypress::Left => "<",
            Keypress::Right => ">",
            Keypress::Activate => "A",
        };
        write!(f, "{}", str)
    }
}

fn print_keypress(keys: &Vec<Keypress>) {
    for k in keys {
        print!("{}", k)
    }
}

const LOCATIONS: &[(usize, usize); KEYS] = &[
    (3, 1), // 0
    (2, 0), // 1
    (2, 1), // 2
    (2, 2), // 3
    (1, 0), // 4
    (1, 1), // 5
    (1, 2), // 6
    (0, 0), // 7
    (0, 1), // 8
    (0, 2), // 9
    (3, 2), // A
];

fn direction(press: &Keypress) -> (usize, usize) {
    match press {
        Keypress::Up => (0, 1),
        Keypress::Down => (1, 1),
        Keypress::Left => (1, 0),
        Keypress::Right => (1, 2),
        Keypress::Activate => (0, 2),
    }
}

const KEYPAD_AVOID: (usize, usize) = (3, 0);
const DPAD_AVOID: (usize, usize) = (0, 0);

fn from_to(current: (usize, usize), target: (usize, usize), mut path: Vec<Keypress>, avoid: (usize, usize), totals_path: &mut Vec<Vec<Keypress>>) {
    if target == current {
        path.push(Keypress::Activate);
        totals_path.push(path);
        return
    } else if current == avoid {
        return
    }

    if let Some((new_h, press)) = Keypress::towards(current.1, target.1, true) {
        let mut new_path = path.clone();
        new_path.push(press);
        from_to((current.0, new_h), target, new_path, avoid, totals_path);
    }
    if let Some((new_v, press)) = Keypress::towards(current.0, target.0, false) {
        path.push(press);
        from_to((new_v, current.1), target, path, avoid, totals_path);
    }
}

fn make_shortest_keypad() -> Array2D<Vec<Vec<Keypress>>> {
    let mut array_map = Array2D::filled_with(Vec::new(), KEYS, KEYS);
    for from in 0..KEYS {
        for to in 0..KEYS {
            let from_pos = LOCATIONS[from];
            let to_pos = LOCATIONS[to];
            let mut path = array_map.get_mut(from, to).unwrap();
            from_to(from_pos, to_pos, Vec::new(), KEYPAD_AVOID, path);
            // println!("\nChecking lengths:");
            // for v in path {
            //     println!("{}", v.len());
            // }
        }
    }

    array_map
}

fn make_shortest_dpad() -> Array2D<Vec<Vec<Keypress>>> {
    let mut array_map = Array2D::filled_with(Vec::new(), DIRS, DIRS);
    for from in 0..DIRS {
        for to in 0..DIRS {
            let from_k = Keypress::try_from(from).unwrap();
            let from_pos = direction(&from_k);
            let to_k = Keypress::try_from(to).unwrap();
            let to_pos = direction(&to_k);
            let mut path = array_map.get_mut(from, to).unwrap();
            from_to(from_pos, to_pos, Vec::new(), DPAD_AVOID, path);
        }
    }
    array_map
}

fn to_digit(c: char) -> usize {
    match c.to_digit(10) {
        Some(u) => u as usize,
        None => 10
    }
}

fn all_routes(chars: Vec<&Vec<Vec<Keypress>>>) -> Vec<Vec<Keypress>> {
    let total = chars.iter().fold(1, |acc, i| acc * i.len());
    let mut long_routes = vec![Vec::new(); total];
    for (j, opts) in chars.iter().enumerate() {
        for i in 0..total {
            let o = opts.get(i % opts.len()).unwrap();
            long_routes[i].extend(o);
        }
    }
    long_routes
}

fn stage_2(route: &Vec<Vec<Keypress>>, dpad_routes: &Array2D<Vec<Vec<Keypress>>>) -> Vec<Vec<Keypress>> {
    let routes = route.iter().map(|number| {
        let mut path = Vec::new();
        let mut from = Keypress::Activate;
        for key in number {
            let from_i : usize = from.into();
            let to_i : usize = (*key).into();
            let mut dpad_route = dpad_routes.get(from_i, to_i).unwrap().iter().min_by_key(|r| r.len()).unwrap().clone();
            dpad_route.push(Keypress::Activate);
            path.extend(dpad_route);
        }
        path
        // path.iter().min_by_key(|r| r.len()).unwrap().clone()
    }).collect::<Vec<_>>();
    // let lens = routes.iter().map(|r| r.len()).collect::<Vec<_>>();
    // let fst = lens.first().unwrap();
    // for len in lens.iter() {
    //     assert_eq!(fst, len);
    // }
    routes
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let keypads = make_shortest_keypad();
    let dpads = make_shortest_dpad();
    let keypads_2_iter = keypads.elements_row_major_iter().map(|el| {
        let snd = stage_2(el, &dpads);
        let third = stage_2(&snd, &dpads);
        third.into_iter().min_by_key(|route| route.len()).unwrap()
    });
    let key_3 = Array2D::from_iter_row_major(keypads_2_iter, KEYS, KEYS).unwrap();
    // let keypads_3 = keypads_2.elements_row_major_iter().map(|el| {
    //     stage_2(el, &dpads)
    // });
    // println!("{:?}", keypads);
    let mut keys = Vec::new();
    let mut from = 10;
    for c in input.first().unwrap() {
        let to = to_digit(*c);
        keys.push(&keypads[(from, to)]);
        from = to;
    }
    let mut ans = Vec::new();
    for line in input {
        let mut from_i = 10;
        let mut counter = 0;
        for c in line.iter() {
            let to_i = to_digit(*c);
            let full_route = key_3.get(from_i, to_i).unwrap();
            println!("{:?}", full_route);
            counter += full_route.len();
        }
        let s: String = String::from_iter(line.iter().take(line.len() - 1));
        let num : usize = s.parse().unwrap();
        // let num : usize = line[0..line.len()-1].into().parse().unwrap();
        println!("{} {}", counter, num);
        ans.push(num * counter)
    }
    ans.iter().sum()
    // let long_routes = all_routes(keys);
    // for route in long_routes.iter() {
    //     println!("\n");
    //     print_keypress(route);
    // }
    //
    // let stage_2: Vec<_> = long_routes.iter().map(|vec_k| {
    //     let mut stages = Vec::new();
    //     let mut from = Keypress::Activate;
    //     for to in vec_k {
    //         let from_i: usize = from.into();
    //         let to_i: usize = (*to).into();
    //         let paths = dpads.get(from_i, to_i).unwrap();
    //         println!("\n This is a path{:?}", paths);
    //         from = *to;
    //         stages.extend(paths);
    //     }
    //     stages
    // } ).collect();
    //
    // println!("{:?}", stage_2);

    // let long_routes_2 = all_routes(stage_2);

    // for k in *keys.first().unwrap() {
    //     print_keypress(k);
    // }

    // todo!("Implement part 1")
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    todo!("Implement part 2")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypress() {
        let (new_h, press) = Keypress::towards(0, 1, true).unwrap();
        assert_eq!(new_h, 1);
        assert_eq!(press, Keypress::Right);
    }

    #[test]
    fn test_keypress_v() {
        let (new_v, press) = Keypress::towards(2, 0, false).unwrap();
        assert_eq!(new_v, 1);
        assert_eq!(press, Keypress::Up);
    }

    #[test]
    fn test_keypress_same() {
        let res = Keypress::towards(2, 2, false);
        assert_eq!(res, None);
    }
}