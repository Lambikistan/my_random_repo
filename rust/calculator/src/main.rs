// Note: from https://www.freecodecamp.org/news/rust-in-replit/#rust-overview
use std::env::{args,Args};

fn main() {
    let mut args: Args = args();
    println!("{:?}", args);

    // using 1 skips the first argument (name of program), and indexes into the first
    //  argument.  after that use the first index
    let first: String = args.nth(1).unwrap();
    let second: String = args.nth(0).unwrap();
    let third: String = args.nth(0).unwrap();
    println!("{}, {}, {}", first, second, third);
    
    let first_number = first.parse::<f32>().unwrap();
    // note this changes ownership from second to operator
    let operator = second.parse::<char>().unwrap();
    // other way to do it:  
    //  let operator = second.chars().next().unwrap();
    let second_number = third.parse::<f32>().unwrap();
    let answer = operate(operator, first_number, second_number);
    println!("{} {} {} = {}", first_number, operator, second_number, answer);
    // or use format to create a string an print just the string
    println!("{}", output(first_number, operator, second_number, answer));

    println!("Hello, world!");
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // Note that no semicolon means return value after operation is successful
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _   => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!( "{} {} {} = {}", first_number, operator, second_number, result)
}
