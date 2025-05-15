mod test_module;
mod test_use_module;
use crate::test_use_module::*;
//different ways of bringing into scope, you can use the above with a specific function/type etc
//like below, or you can combine them using the final statement
//use std::io;
//use std::io::Write;
//use std::io::{self, Write};

//inclusion of created libraries must be addd to the cargo.toml then included
use restaurant::eat_at_restaurant;

fn main() {
    //Packages: A Cargo feature that lets you build, test, and share crates
    //Crates: A tree of modules that produces a library or executable
    //Modules and use: Let you control the organization, scope, and privacy of paths
    //Paths: A way of naming an item, such as a struct, function, or module
    //For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces -> Chap 14

    //crate is usually referring to a library but technically a program is a crate too its just a binary crate
    //a package is a bundle of multiple crates, it contains a cargo.toml 
    //packages can have as many binary crates but only one library crate
    
    //if you include a module with the mod keyword all its code is included, code of a module is private from its parent module by default, otherwise
    //declare it as pub
    //you can use the 'use' keyword instead of mod to bring that into scope so instead of having to go down the scope tree you can just use the type by name
    //Asparagus instead of crate::garden::vegetables::Asparagus (given that you had 'use crate::garden::vegetables::Asparagus')
    test_module::test_module_fn();
    test_use_module_fn();

    //you can create a library with the following command, the library will 
    //cargo new library_name --lib

    //structs/enums can be made pub as well, if the struct/enum is public the fields are still private by default and must also be declared otherwise

    eat_at_restaurant();
}
