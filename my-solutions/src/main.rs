mod days;

fn main() -> () {
    println!("Hello, world!");
    let app = rudvent_lib::cli::AppBuilder::new(days::DAYS_VEC).with_year(2023).build();
    app.run()
}
