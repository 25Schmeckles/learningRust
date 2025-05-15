fn main() {
    println!("{}",length_of_longest_substring("Hello, world!".to_string()));
}
pub fn length_of_longest_substring(s: String) -> i32 {
        s.len() as i32
    }
