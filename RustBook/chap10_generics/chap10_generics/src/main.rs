//looking at generics like Option, Vec, HashMap and Result
//helps to factor out code for many things but a big one is functions that have the same code but different types we can generalize it with generics
//we can then breakout specific beahavior per type by using traits
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/
//this doesnt work because '>' is not defined for all types that could possible be used, this can be restricted to only certain types or we need a different implementation

//for struct definitions by using different types (T and U we can mix and match as needed!)
struct PointMixMatch<T, U> {
    x: T,
    y: U,
}

//we can also do generic Enums, think about option, and result!
/*
enum Option<T> {
    Some(T),
    None,
}
*/
//here we use the example of matched types so we can look at implementation for specified types
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//implemented for the f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    //each mixes the types differently and still work!
    let both_integer = PointMixMatch { x: 5, y: 10 };
    let both_float = PointMixMatch { x: 1.0, y: 4.0 };
    let integer_and_float = PointMixMatch { x: 5, y: 4.0 };
    //type specific
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    //type generic
    /*
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
    */
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}


