use crate::utils;
use ::bounded_vec_deque::BoundedVecDeque;
pub fn ans_a() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let ints: Vec<_> = str_lines.map(|s| s.parse::<i32>().unwrap()).collect();
    let mut counter = 0;
    let mut last_depth = None;
    for depth in ints {
        if last_depth != None && depth > last_depth.unwrap() {
            counter += 1;
        }
        last_depth = Some(depth);
    }

    counter
}

pub fn ans_b() -> i32 {
    let buffer_len = 3;
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let ints: Vec<_> = str_lines.map(|s| s.parse::<i32>().unwrap()).collect();

    let mut counter = 0;
    let mut buffer: BoundedVecDeque<i32> = BoundedVecDeque::new(buffer_len);
    let (left, right) = ints.split_at(buffer_len);

    for &d in left {
        buffer.push_front(d);
    }
    let mut depth_sum: i32 = left.iter().sum();
    println!("Sum: {}", depth_sum);

    for &depth in right {
        //old_value = buffer.pop_back().unwrap();
        let new_rolling_depth = depth_sum - buffer.pop_back().unwrap() + depth;
        buffer.push_front(depth);
        println!("{}", new_rolling_depth);

        if new_rolling_depth > depth_sum {
            counter += 1;
        }
        depth_sum = new_rolling_depth;
    }

    counter
}
