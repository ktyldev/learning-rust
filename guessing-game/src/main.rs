extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

const MIN: i32 = 1;
const MAX: i32 = 100;

enum Guesser {
    Human,
    Computer
}

fn main() -> io::Result<()> {
    match get_player() {
        Guesser::Human => human_game(),
        Guesser::Computer => computer_game()
    }
}

fn get_player() -> Guesser {
    println!("who's guessing? [h]uman or [c]omputer?");

    loop {
        match player_input().trim() {
            "h" => break Guesser::Human,
            "c" => break Guesser::Computer,
            _ => println!("not a valid option")
        } 
    }
}

fn computer_game() -> io::Result<()> {
    println!("Think of a number between {} and {}", MIN, MAX);

    let mut bounds = [MIN, MAX];
    loop {
        let guess = (bounds[0] + bounds[1]) / 2;

        println!("is it {}?", guess);
        println!("too [h]igh, too [l]ow, [c]orrect");

        match player_input().trim() {
            "h" => { bounds[1] = guess; },
            "l" => { bounds[0] = guess; },
            "c" => {
                println!("yay!");
                break Ok(());
            },
            _ => println!("not a valid option")
        }
    }
}

fn human_game() -> io::Result<()> {
    let num = rand::thread_rng().gen_range(MIN, MAX);
    'main: loop {
        if compare_guess(num, player_guess()) {
            break 'main; 
        }
    }

    Ok(())
}

fn compare_guess(num: i32, guess: i32) -> bool {
    match guess.cmp(&num) {
        Ordering::Less => println!("too low"),
        Ordering::Greater => println!("too high"),
        Ordering::Equal => {
            println!("you win! the number was {}", num); 
            return true;
        }
    }

    false
}

fn player_guess() -> i32 {
    println!("guess the number between {} and {}: ", MIN, MAX);

    loop {
        match player_input().trim().parse::<i32>() {
            Ok(i) => break i,
            Err(_) => println!("couldn't parse input, try again")
        }
    }
}

fn player_input() -> String {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => return input,
            Err(e) => println!("{}", e)
        } 
    }
}
