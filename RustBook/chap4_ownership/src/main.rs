fn main() {
    //hardcoded non expandable string on stack
    let _s = "hello";
    // string on heap
    let _s = String::from("hello");
    // :: operator is allowing us to use from() without including the entire library
    let s1 = String::from("hello");
    //s1 is now out of scope so it is no longer valid!
    let mut s2 = s1;
    s2.push_str(", world!"); // push_str() appends a literal to a String
    //memory is freed when the variable goes out of scope
    //freeing is called drop() in rust
    //there are copy functions that you can use like .clone()
    
    //you can shadow it since its no longer in scope
    let s1 = s2.clone();
    //if a type is smaller and trivial, you dont need to use a copy method, to be sure you must check with the Type, these implement copy:
    //All the integer types, such as u32.
    //The Boolean type, bool, with values true and false.
    //All the floating-point types, such as f64.
    //The character type, char.
    //Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    //this same relationship is maintained when passed to functions, if the variable implements copy, it is passed by value, if it does not it is passed by reference(kind of, there is real pass by reference up next) and the original goes out of scope
    //you can also shadow it right back into its original variable by returning into it

    let mut s1 = takes_and_gives_back(s1);
    println!("s1 = {s1}, s2 = {s2}");

    //now we look at pointers and passing them by reference
    //this is great because it will not go out of scope, however you CANNOT change the values of a pass by reference if it is not Mut
    //in fact to change it both the variable and the reference must be mutable
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    change(&mut s1);
    println!("{s1}");
    //if you have a mutable reference to a value, you can have no other references to that value
    //you can control this by manipulating scope like below
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    //if you have immutable references you must use them at least once before creating a mutable one
    //using it through a function can be tracked and it will go out of scope after last use if not used it will panic

    let r2 = &mut s;

    //slices are a type that link enumeration to the variable, so if the variable changes later so will the result
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
   
   
    //we can return a slice to do what is explained above
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}",word);
}


fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
    
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
//first word as normally done, not linked to the actual variable
fn first_word_noslice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
//returning a slice to findt the first word
//fn first_word(s: &String) -> &str {

//this header is better because it allows deref coercion from stack to heap so will allow str and Strings to be passed
fn first_word(s: &str) -> &str {    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
