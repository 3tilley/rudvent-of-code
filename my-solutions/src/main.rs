use rudvent_lib::types::SolutionBuilders;

mod days;

static vec: SolutionBuilders = days::DAYS_VEC;

fn main() -> () {
    println!("Hello, world!");
    let app = rudvent_lib::cli::AppBuilder::new(&vec)
        .with_year(2023)
        .build();
    app.run().unwrap();
}
