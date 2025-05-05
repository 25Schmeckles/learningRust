fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    //you can define an instance of a struct from another
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //this draws from user2 because user1 has been moved and is out of scope now!
    let user3 = User {
        email: String::from("another@example.com"),
        //you can specify whatever fields you like and then take the rest from the struct specified
        ..user2
    };
    //but there is good shorthand like this
    //also, if a struct has any element that does not implement copy then the whole struct does not
    //using this shorthand or copying any element will take the old one out of scope
    //we could switch to String instead of &str which implements copy and we would be all set

    //there are also Tuple-structs, no name for each field just values
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //accessing tuple-struct values
    //same as regular tuple indexing
    println!("R: {}", black.0);

    //you can also have a unit-like struct with no values
    let subject = AlwaysEqual;

    //looking at methods now
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //call on the constructor
    let sq = Rectangle::square(3);

}

//you can return a struct like any data type
//we also have this nice shorthand so we dont have to set a field to the input
//if the names match then this works!
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
//define struct outside main with however many elements you like, are referenced by User.element
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like struct
struct AlwaysEqual;

//define the struct
struct Rectangle {
    width: u32,
    height: u32,
}
//implementing methods for the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    //method that takes another struct as an argument, this can even be any other type
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //function that returns an object which is why it has no &self call
    //but does have Self which aliases Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}