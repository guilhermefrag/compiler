mod my_module;

use my_module::my_function;

fn main() {
    my_function();
    let values_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let result = binary_search_three(values_array, target);
    println!("The result is: {}", result);
}

pub fn binary_search_three(values_array: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = values_array.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if values_array[mid as usize] == target {
            return mid;
        }
        if values_array[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1

}