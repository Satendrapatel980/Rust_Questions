fn is_palindrome(s: &str) -> bool {
    let s = s.trim().to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let test_string1 = "racecar"; // This is a palindrome.
    let test_string2 = "hello";   // This is not a palindrome.
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}
