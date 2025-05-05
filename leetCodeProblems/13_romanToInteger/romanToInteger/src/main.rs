use std::collections::HashMap;


fn main() {
    let s = String::from("VIVXCDM");
    roman_to_int(s);
}
pub fn roman_to_int(s: String) -> i32 {
    let numerals = HashMap::from([('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)]);
    let output: Vec<i32> = s
        .chars()
        .map(|ch| numerals[&ch])
        .collect();
    
    
    let as_string: String = output
        .iter()
        .map(|n| n.to_string())
        .collect::<String>();
    println!("{}",as_string);
    



    return 5
}