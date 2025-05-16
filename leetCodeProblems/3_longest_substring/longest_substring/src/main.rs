use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied,Vacant};
fn main() {
    println!("{}",length_of_longest_substring("abcabcabc".to_string()));
}
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut sub_strings = HashMap::new();
    for mut i in 0..s.len() {
        for mut j in i + 1..=s.len() {
            println!("{}", &s[i..j]);
            let key = &s[i..j];
            match sub_strings.entry(key) {
                Occupied(mut entry) => {
                    println!("Key already exists with value: {}", entry.get());
                    i = j+1;
                    j = i+1;
                }
                Vacant(entry) => {
                    println!("Inserting new key");
                    entry.insert(j-i);
                }
            }
        }
    }
    if let Some(max_val) = sub_strings.values().max() {
        return *max_val as i32;
    }
    -1
}
//needs work, currently puts in all substrings, i can probably do something with this