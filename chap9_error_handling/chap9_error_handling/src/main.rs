    //recoverable vs unrecoverable errors, Result Type (for recoverable) and panic! macro (for unrecoverable)
    //panic! will print a message then safely exit the program, you can manually call it and give it the message you want it to display
    //if in the [profile] section of cargo.toml you put "panic = 'abort'"
    //instead of cleaning up the stack it will just immediately quit and leave it to the OS, this will save some room in your binary
    //we can backtrace a panic!
    //RUST_BACKTRACE=anyvaluebut0 cargo run
    //will run with the backtrace displayed
    //
    //Result is an Enum with Ok and Err
use std::fs::File;

//if it returns Err, it actually doesn't panic or do anything by default but it does because we specifically told it to panic!
//you could in the Err branch, create the file, but also creating returns a Result so you can check if it's successful, shown below
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
//runs even though the file doesnt exist

/*
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
*/
//this can be paired with closures (chapter 13) for more concise code like so:
/*
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
*/
//many methods have built in behavior like this, like unwrap which if it fails panics
//you can also return the error to the calling function "propagating it" for more robust behavior, just need the Result as the return type
/*
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/
//as a shortcut we have the ? operator

/*
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
*/
//this actually already has standard implementation so can just be done as
/*
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
*/
//? can only be used when it returns the proper type, Result or Option or another implementation of FromResidual
//main has limited return types but can return a Result
/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/
//Box<dyn Error> is a trait object, (chapter 18)
//this will allow return types of all errors
//Binary will return 0 if main returns Ok() and nonzero for anything else

//when to panic!
//in general best to return results to be handled downstream which can panic at that point, when writing prototype code its best to have it panic so we can immediately find issues as we test
//panic if:
//it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

//The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
//Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
//There’s not a good way to encode this information in the types you use. (example in chapter 18)

//in my head we can wrap everything in Result and then hide all the error handling in its own function so my functions can stay about their job!
//instead of everything being a Result we can actually make our own custom types that do this for us!

/*
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
*/

//big take aways, panic when its harmful or unexpected, otherwise propogate the error downstream, then if I used Result or my own types or a combination I can move all my error handling to its own place
//so i have very clean and clear code!