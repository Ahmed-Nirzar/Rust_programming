// Celsius to Fehrenheit and Fehrenheit to Celsius.

use std::io;

fn main() {
    println!("To convert Fehrenheit to Celsius press '1' and to do reverse press '2'");
    
    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");

    let choice: u32 = choice
    .trim()
    .parse() 
    .expect("Please type a number.");

    if choice == 1 {

        println!("Enter the number: ");
        let mut val = String::new();

        io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");

        let val: f32 = val
        .trim()
        .parse() 
        .expect("Please type a number.");

        let mut c: f32 = val-32.0;
        c = c*5.0;
        c = c/9.0;

        println!("Celsius to Fehrenheit : {c}.");

    };

    if choice == 2 {

        println!("Enter the number: ");
        let mut val = String::new();

        io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");

        let val: f32 = val
        .trim()
        .parse() 
        .expect("Please type a number.");

        let f: f32 = ((val*9.0)/5.0) + 32.0;

        println!("Celsius to Fehrenheit : {f}.");

    };
}