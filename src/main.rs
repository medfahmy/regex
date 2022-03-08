use std::io::{self, BufRead, Write};

mod turnstile;
use turnstile::*;

fn main() {
    println!("hi mom");

    let mut state = State::Locked;

    // let arr = [1, 2, 4];
    // println!("{}", arr[state as usize]);

    println!("State: {:?}", state);

    print!("> ");
    match io::stdout().flush() {
        _result => {
            for line in io::stdin().lock().lines() {
                match line.unwrap().as_str() {
                    "push" => state = next_state(state, Event::Push),
                    "coin" => state = turnstile::next_state(state, Event::Coin),
                    "quit" => return,
                    unknown => eprintln!("ERROR: unknown event {}", unknown)
                }

                println!("{:?}", state);
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
    }
}