fn main() {
    let string = String::from("a");
    println!("{}",is_palindrome(string));
}
fn is_palindrome(s: String) -> bool 
{
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    for i in 0..(len / 2) {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}
//create function that checks if arbitrary string is a palindrome, checking middle to out,
//then the driving function will have to rattle off the strings searching by middle character
//starting at the middle then moving outward

//maybe not, maybe search starting in the middle of a word then check moving outward?