fn kth_smallest(nums: &[i32], k: usize) -> Option<i32> {
    nums.iter().cloned().nth(k - 1)
}

fn main() {
    let nums = vec![3, 5, 2, 8, 1, 9];
    let k = 3;
    match kth_smallest(&nums, k) {
        Some(num) => println!("The {}th smallest element is: {}", k, num),
        None => println!("Invalid input"),
    }
}
