fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9];
    let median = find_median(&nums);
    println!("Median: {}", median);
}
