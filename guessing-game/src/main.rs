extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

const MIN: i32 = 1;
const MAX: i32 = 100;

fn main() -> io::Result<()> {
    game(rand::thread_rng().gen_range(MIN, MAX))
}

fn game(num: i32) -> io::Result<()> {
    loop {
        match guess() {
            Ok(g) => {
                match g.cmp(&num) {
                    Ordering::Less => println!("too low"),
                    Ordering::Equal => {
                        println!("you win! the number was {}", num);
                        break Ok(())     
                    },
                    Ordering::Greater => println!("too high")
                }
            }
            Err(e) => println!("error: {:?}", e)
        }
    }
}

fn guess() -> Result<i32, io::Error> {
    println!("guess the number between {} and {}: ", MIN, MAX);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim()
        .parse()
        .unwrap())
}
