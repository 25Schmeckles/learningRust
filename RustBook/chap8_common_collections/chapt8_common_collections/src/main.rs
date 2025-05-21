fn main() {
    //looking at heap stored variants of vector, string and hash maps
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //accessing vector values has 2 ways
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //iterating can be done in multiple ways, non mutable reference means i can only access values even if the vector is mut
    for i in &v {
        println!("{i}");
    }
    //or like this if i want to change the values
    for i in &mut v {
        *i += 50;
    }
    //we can use enums for storing different types
    //Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled (Chapter 6)
    //If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object (Chapter 18)


    //Strings
    let data = "initial contents";
    let s = data.to_string();
    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    //you could bypass this and just directly use
    let s = String::from("initial contents");
    //String is UTF-8 so supports all kinds of things

    //you can concatenate multiple ways
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}",s);
    //or with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}",s3);
    //doing this with multiple strings becomes weird because of the borrow of s1
    //we can use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}",s);
    //String is actually a wrapper of a Vec<u8> because it implements UTF-8
    //because of this and because valid characters may take up more than one "slot" indexing must be done via slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s);
    //since depending on how crazy you're getting with UTF-8 looping may be a little more complex so the typical methods are
    for c in "Зд".chars() {
    println!("{c}");
    }
    //or
    for b in "Зд".bytes() {
    println!("{b}");
    }
    //since letters could be more than one value and grapheme clusters can be even more complicated there is some extra work that may need to be done

    //Hash maps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //acessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //standard looping
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    //For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    //For owned values like String, the values will be moved and the hash map will be the owner of those values

    //if you want to overwrite values in a hash you can do that
    scores.insert(String::from("Blue"), 25);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    //you can also use .or_insert to only do it if no value exists
    scores.entry(String::from("Blue")).or_insert(10);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    //changing value while looking at the old value
    //looks at text and creates a map of how many times each word appears
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
