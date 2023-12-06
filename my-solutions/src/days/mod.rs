use rudvent_lib::types::SolutionBuilders;

// Begin mod declarations
mod day_1;
mod day_2;
mod day_23;
mod day_3;
mod day_4;
mod day_5;
// End mod declarations

// pub static TEST_VEC: &'static [Option<Box<dyn SolutionBuilder>>] = &[
//     Some(Box::new(crate::templates::day_template::make_sol())),
// ];

pub static DAYS_VEC: SolutionBuilders = &[
    // Begin: Add days
    Some(day_1::make_sol),  // Day 01
    Some(day_2::make_sol),  // Day 02
    Some(day_3::make_sol),  // Day 03
    Some(day_4::make_sol),  // Day 04
    Some(day_5::make_sol),  // Day 05
    None,                   // Day 06
    None,                   // Day 07
    None,                   // Day 08
    None,                   // Day 09
    None,                   // Day 10
    None,                   // Day 11
    None,                   // Day 12
    None,                   // Day 13
    None,                   // Day 14
    None,                   // Day 15
    None,                   // Day 16
    None,                   // Day 17
    None,                   // Day 18
    None,                   // Day 19
    None,                   // Day 20
    None,                   // Day 21
    None,                   // Day 22
    Some(day_23::make_sol), // Day 23
    None,                   // Day 24
    None,                   // Day 25
                            // End: Add days
];

// static DAYS_CHECK : Vec<Box<dyn Solution>> = vec![
//     day_template::make_sol(1),
// ];
