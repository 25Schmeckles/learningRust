use std::collections::HashMap;

fn main() {
    //let s = String::from("iXX");
    let answer = roman_to_int("III".to_uppercase());
    println!("{}",answer);
    let answer = roman_to_int("LVIII".to_uppercase());
    println!("{}",answer);
    let answer = roman_to_int("MCMXCIV".to_uppercase());
    println!("{}",answer);
}
pub fn roman_to_int(s: String) -> i32 {
    let numerals = HashMap::from([('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)]);
    //convert from numeral to vector of numbers
    let mut numbers: Vec<i32> = s
        .chars()
        .map(|ch| numerals[&ch])
        .collect();
    //if the next value is bigger than previous flip the sign
        for i in 0..numbers.len() - 1 {
            if numbers[i + 1] > numbers[i] {
                numbers[i] = -numbers[i];
            }
        }
    return numbers.iter().sum();
}