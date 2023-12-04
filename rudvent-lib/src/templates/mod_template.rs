use crate::solution::SolutionBuilder;
use crate::types::SolutionBuilders;

// Begin mod declarations
// End mod declarations

pub static TEST_VEC: &'static [Option<Box<dyn SolutionBuilder>>] = &[
    Some(Box::new(crate::templates::day_template::make_sol())),
];

pub static DAYS_VEC: SolutionBuilders = &[
    // Begin: Add days
    Some(crate::templates::day_template::make_sol()),
    None, // Day 01
    None, // Day 02
    None, // Day 03
    None, // Day 04
    None, // Day 05
    None, // Day 06
    None, // Day 07
    None, // Day 08
    None, // Day 09
    None, // Day 10
    None, // Day 11
    None, // Day 12
    None, // Day 13
    None, // Day 14
    None, // Day 15
    None, // Day 16
    None, // Day 17
    None, // Day 18
    None, // Day 19
    None, // Day 20
    None, // Day 21
    None, // Day 22
    None, // Day 23
    None, // Day 24
    None, // Day 25
    // End: Add days
];

// static DAYS_CHECK : Vec<Box<dyn Solution>> = vec![
//     day_template::make_sol(1),
// ];