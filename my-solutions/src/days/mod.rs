use rudvent_lib::types::SolutionBuilders;

// Begin mod declarations
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
// End mod declarations

// pub static TEST_VEC: &'static [Option<Box<dyn SolutionBuilder>>] = &[
//     Some(Box::new(crate::templates::day_template::make_sol())),
// ];

pub static DAYS_ARRAY: SolutionBuilders = &[
    // Begin: Add days
    Some(day_1::make_sol), // Day 01
    Some(day_2::make_sol), // Day 02
    Some(day_3::make_sol), // Day 03
    Some(day_4::make_sol), // Day 04
    Some(day_5::make_sol), // Day 05
    Some(day_6::make_sol), // Day 06
    Some(day_7::make_sol), // Day 07
    Some(day_8::make_sol), // Day 08
    Some(day_9::make_sol), // Day 09
    Some(day_10::make_sol),                  // Day 10
    Some(day_11::make_sol),                  // Day 11
    Some(day_12::make_sol),                  // Day 12
    Some(day_13::make_sol),                  // Day 13
    None,                  // Day 14
    None,                  // Day 15
    None,                  // Day 16
    None,                  // Day 17
    None,                  // Day 18
    None,                  // Day 19
    None,                  // Day 20
    None,                  // Day 21
    None,                  // Day 22
    None,                  // Day 23
    None,                  // Day 24
    None,                  // Day 25
                           // End: Add days
];