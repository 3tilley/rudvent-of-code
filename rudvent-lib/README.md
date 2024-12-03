# rudvent-lib

Solve Advent of Code from your terminal! This library won't give you any solutions, or provide utilities to solve any problems, but it will generate templates to make it as boilerplate-free as possible.

## Setting up your solution

Add this library as a dependency (from git for now), create a `main.rs` that looks like the below (or copy [the example](https://github.com/3tilley/rudvent-of-code/tree/main/rudvent-lib/examples/start_here)), and let rudvent-lib get data and submit your solutions for you.

```rust
use rudvent_lib::types::SolutionBuilders;

// Comment this out when you first make your main, as rudvent-lib hasn't created your mod files yet
// mod days;
// static SOLUTIONS_ARRAY: SolutionBuilders = days::DAYS_VEC;

// Comment or delete this after you've run `new` for any day and `days/mod.rs` has been created
static SOLUTIONS_ARRAY: SolutionBuilders = &[];

fn main() -> () {
    let app = rudvent_lib::cli::AppBuilder::new(&SOLUTIONS_ARRAY)
        .with_year(2024)
        .build();
    app.run().unwrap();
}
```

## Solving each day
With `main.rs` set up, you can run `new` and `fetch` to populate a local cache and solve the problem:

![image](https://github.com/3tilley/rudvent-of-code/assets/1138504/d3961cb3-3938-44b1-b181-b6e54c4a1a39)

This will generate you a template for a new day like the below, where you can fill in the types and the gaps.

```rust
// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<u64>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 0;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    for line in input.lines() {
        todo!()
    }
    vec![1, 2, 3]
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
) -> OutputPart1 {
    todo!("Implement part 1")
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
) -> OutputPart1 {
    todo!("Implement part 2")
}
```


Features:
- [x] Fetch the problem description from the site
- [x] Generate a CLI with `solve`, `new`, `fetch`, `desc` to run everything from the terminal
- [x] Check the user solution agrees with the example
- [x] Output timing data
- [ ] Fetch data automatically on prompt after run
- [ ] UserParams from extra args
- [ ] Benches
- [ ] Fetch example 2 properly. This might need a timeout and a check to make sure the db is updated
- [ ] Warn if example file needs filling in manually
- [ ] Desc to support part 1 and part 2
