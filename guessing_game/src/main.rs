//! A guessing-the-number game implementation.

extern crate guessing_game;

use std::old_io;
use std::rand;
use std::cmp::Ordering;
use guessing_game::compare;

fn main () {
    println!("Guess a number!");

    let secret_number = (rand::random::<u32>() % 100) + 1;

loop {
        println!("Please input your guess.");

        let input = old_io::stdin().read_line()
                                .ok()
                                .expect("Failed to read line");

        let input_num: Result<u32, _> = input.trim().parse();

        let num = match input_num {
            Ok(n) => n,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {:?}", num);

        match compare::cmp(num, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            },
        }
    }
}


#[cfg(test)]
mod tests {
    
    use guessing_game::compare;
    use std::cmp::Ordering;
    
    #[test]
    fn main_level_test() {
        assert_eq!(Ordering::Less, compare::cmp(2, 3));
    }
    
    #[test]
    #[should_fail(expected = "assertion failed")]
    fn main_level_fail_check() {
        assert_eq!(Ordering::Equal, compare::cmp(1, 2));
    }
}