// Exercise: Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often) of the list.
use std::collections::HashMap;

fn main() {
    let nums = [-5, -3, -3, -3, 0, 0, 12, 99, 101];

    let median = get_median(&nums);
    println!("The median is {}", median);

    let mode = get_mode(&nums);
    println!("The mode is {}", mode);
}

fn get_mode(nums: &[i32]) -> i32 {
    let mut num_to_frequency = HashMap::new();
    for num in nums {
        *num_to_frequency.entry(num).or_insert(0) += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in num_to_frequency {
        if value > max_count {
            max_count = value;
            mode = *key;
        }
    }

    mode
}

fn get_median(nums: &[i32]) -> f32 {
    let mut median = 0.0;

    if nums.len() % 2 != 0 {
        let middle_index = nums.len() / 2;
        median = nums[middle_index] as f32;
    } else {
        let upper_middle_index = nums.len() / 2;
        let lower_middle_index = upper_middle_index - 1;
        median = (nums[lower_middle_index] + nums[upper_middle_index]) as f32 / 2.0;
    }

    median
}
