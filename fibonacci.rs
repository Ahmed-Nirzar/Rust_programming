// Finding Fibonacci number under 94.

use std::io;

fn main() {

    let mut p2: u128 = 2;
    let mut pn: u128 = 1+p2;
    let mut p3: u128 = pn;
    let mut i = 3;
    let mut val = String::new();

    println!("Enter a number (more than 3 and less than 95). ");
    
    io::stdin()
    .read_line(&mut val)
    .expect("Failed to read line");

    let val: u32 = val
    .trim()
    .parse() 
    .expect("Please type a number.");

    loop {
        
        i = i+1;
        pn = pn+p2;
        p2 = pn;
        pn = pn+p3;
        p3 = pn;

        if i == val {
            println!("The {i}th Fibonacci number is {pn}.");
            break;
        };
        

        
    }
}