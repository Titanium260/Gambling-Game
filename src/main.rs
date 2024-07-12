use rand::{Rng};
use std::io::stdin;
use std::thread::sleep;
use std::time;

fn main() {

    loop {
    
        let mut rng = rand::thread_rng();

        let y: i64 = rng.gen_range(1..100);

        let mut input_line = String::new();
    
        println!("Enter your number in range of 1-100... (Ctrl + C to exit)");

        stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        let x: i64 = input_line.trim().parse().expect("Input not an integer");

        if x == y {

            println!("You won! RNJesus loves you!");
            
        } else {

            println!("You failed! Correct number was : {}", y);

        }

        sleep(time::Duration::from_millis(1500));

    }

}
