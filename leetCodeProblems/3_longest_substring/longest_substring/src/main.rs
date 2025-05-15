fn main() {
    println!("{}",length_of_longest_substring("abcabcbb".to_string()));
}
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 1;
    let mut current_max = 1;
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < s.len() - 1 {
        if s[i] == s[i+1] {
            println!("{}={}",s[i],s[i+1]);
            current_max = 1;
        }
        else {
            println!("{}!{}",s[i],s[i+1]);
            current_max +=1;
        }
        if max_len < current_max {
            max_len = current_max; 
        }
        i += 1;
    }
    return max_len as i32;
}
//idea, put all substrings into hashmap of substring -> length pairs, then return the max length, runtime is exponential so this is sad only thing i can think of to shorten it is if there is
//some condition where i know a repetition has happened to continuing wont help but by that point ive already done the exponential part so maybe there is no way out of this?