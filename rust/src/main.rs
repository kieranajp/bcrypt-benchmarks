extern crate bcrypt;

use bcrypt::{verify};
// use std::env;

fn main() {
    // Prints each argument on a separate line
    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    // let hashed = match hash("hunter2", DEFAULT_COST) {
    //     Ok(h) => h,
    //     Err(_) => panic!()
    // };



    let valid = match verify("abc123", "$2y$10$VocDgtIzt4xuq.mH4o1YJe0UJscyf/EuEMW84Bq35eVJXz685ApUO") {
        Ok(valid) => valid,
        Err(_) => panic!()
    };

    println!("{}", valid);
}
