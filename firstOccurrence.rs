fn find_first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(index);
        }
        else if num > target {
            break;
        }
    }
    None
}

fn main() {
    let nums = vec![1, 2, 3, 3, 5, 6, 7, 8, 9]; // Example sorted array
    let target = 3; // Target number to search for
    match find_first_occurrence(&nums, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
