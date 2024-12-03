#![allow(warnings)]
use rudvent_lib::types::SolutionBuilders;

// Comment this out when you first make your main, as rudvent-lib hasn't created your mod files yet
mod days;
static SOLUTIONS_ARRAY: SolutionBuilders = days::DAYS_ARRAY;

// Comment or delete this after you've run `new` for any day and `days/mod.rs` has been created
//static SOLUTIONS_ARRAY: SolutionBuilders = &[];

fn main() -> () {
    let app = rudvent_lib::cli::AppBuilder::new(&SOLUTIONS_ARRAY)
        .with_year(2024)
        .build();
    app.run().unwrap();
}
