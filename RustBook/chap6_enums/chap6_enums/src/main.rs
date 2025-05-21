#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    //Where structs give you a way of grouping together related fields and data, 
    // like a Rectangle with its width and height, enums give you a way of saying
    // a value is one of a possible set of values
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    //we can also make methods for enums with an impl block just like structs
    //Option enum is the monad wrapper
    /*Definition, is always included
    enum Option<T> {
    None,
    Some(T),
    }
    */
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    //often you get Nones from "asking an invalid question" with your method, the methods
    //often return Option<T> which we can then match/if to continue

    //calling the match function while passing in the enum type constructor
    //this is a great pattern that should be utilized frequently to take
    //advantage of enum/matching relationship
    //match must cover all possible cases including None or it won't compile
    //there is also the "other" keyword for a catch all, or '_' if you don't 
    //need to actually use the value
    //you can also have it do nothing by returning the unit-value
    //'_ => ()'
    /*
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
*/
value_in_cents(Coin::Quarter(UsState::Alaska));

//if let and let else
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
//can be replaced with this to reduce boilerplate
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
//you can also add an else to this pattern, if your match has 1 valid match and the catch all does something rather than nothing

let coin = Coin::Quarter(UsState::Alaska);
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
//can be using the let else pattern
let coin = Coin::Quarter(UsState::Alaska);
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
//depending on control flow, just a regular if, else if, else statement may make it clearer

}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
//we can make functions that operate on option<t> and return option <t> and use match to disambiguate away Nones
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}