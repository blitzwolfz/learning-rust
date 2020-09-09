use std::io; //Used for io in Rust

fn main() {
    
    //Single Line comment

    /*Multi 
    line 
    commenting */

    //Basic print statements
    println!("Hello World, what is your name?");

    //All variables are non mutiable, and have to be explicity stated otherwise
    let creator = "blitzwolfz";

    //mut means the var can be changed
    let mut name = String::new();

    //io::stdin() means it takes something from the console
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    //Print states use {} for formatting
    //stdin has a trailing whitespace issue .trim_end_matches(&['\r', '\n'][..]) this is the solution
    println!("Hello there {}, my creator's name is {}", name.trim_end_matches(&['\r', '\n'][..]), creator);

    let mut pause = String::new();

    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");

    println!("Thanks!: {}", pause.trim_end_matches(&['\r', '\n'][..]));
}
