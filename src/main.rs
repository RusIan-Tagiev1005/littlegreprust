use regex::Regex;
use std::io;
use std::env;


fn main() {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Invalid input");
    println!("{}", user_input);

}

// -h -f -n
// grepr -f file regex
// grepr -n dir regex
//
