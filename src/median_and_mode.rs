/*
* Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
* and mode (the value that occurs most often; a hash map will be helpful here) of the list
*/

use std::collections::HashMap;

pub fn get_median (mut list: Vec<i32>) -> f32 {

    // median
    list.sort();
    let length = list.len();
    let isEven = length % 2 == 0;
    
    match isEven {
        true => (list[length / 2] as f32 + list[(length / 2) - 1] as f32) / 2.0,
        false => list[length / 2] as f32
    }
}

fn get_mode (mut list: Vec<i32>) {
    
}
