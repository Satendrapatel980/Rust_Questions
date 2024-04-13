fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let words = "The quick brown fox jumps over the lazy dog";
    
    match shortest_word(words) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found"),
    }
}
