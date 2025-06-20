fn main() {
    println!("{}",longest_palindrome("abcdd".to_string()));
}
//normal method (expand around center) is O(N^2) worst case, will add Manacher's algorithm which is O(n)
fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut best = "";

    for center in 0..len {
        // Odd-length palindromes
        let mut l = center;
        let mut r = center;
        while l > 0 && r < len - 1 {
            if chars[l - 1] == chars[r + 1] {
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }
        if r - l + 1 > best.len() {
            best = &s[l..=r];
        }
        // Even-length palindromes (double letter center)
        if center + 1 < len && chars[center] == chars[center + 1] {
            l = center;
            r = center + 1;
            while l > 0 && r < len - 1 {
                if chars[l - 1] == chars[r + 1] {
                    l -= 1;
                    r += 1;
                } else {
                    break;
                }
            }
            if r - l + 1 > best.len() {
                best = &s[l..=r];
            }
        }
    }

    best.to_string()
}

fn is_palindrome(s: &[char]) -> bool {
    s.iter().eq(s.iter().rev())
}