/*
* Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
* and mode (the value that occurs most often; a hash map will be helpful here) of the list
*/

use std::collections::HashMap;

pub fn get_median (mut list: Vec<i32>) -> f32 {

    // median
    list.sort();
    let length = list.len();
    let is_even = length % 2 == 0;
    
    match is_even {
        true => (list[length / 2] as f32 + list[(length / 2) - 1] as f32) / 2.0,
        false => list[length / 2] as f32
    }
}

pub fn get_mode (list: Vec<i32>) -> Vec<i32> {
    let mut num_map = HashMap::new();

    // populate map
    for &number in &list {
        let count = num_map.entry(number).or_insert(0);
        *count += 1
    }

    // Find the highest count
    let max_count = num_map.values().cloned().max().unwrap_or(0);

    // Collect all numbers with the highest count
    num_map.into_iter()
           .filter(|&(_, count)| count == max_count)
           .map(|(num, _)| num)
           .collect()
}
