#![allow(dead_code)]

use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Locked,
    Unlocked,
}

#[derive(Debug)]
pub enum Event {
    Push,
    Coin,
}

use State::*;

const STATES_COUNT: usize = 2;
const EVENTS_COUNT: usize = 2;

const FSM: [[State; EVENTS_COUNT]; STATES_COUNT] = [
    [Locked, Unlocked],
    [Locked, Unlocked]
];

// fn next_state(state: State, event: Event) -> State {
//     match event {
//         Push => Locked,
//         Coin => Unlocked,
//     }
// }

pub fn next_state(state: State, event: Event) -> State {
    FSM[state as usize][event as usize]
}

pub fn play() {
    let mut state = State::Locked;

    println!("State: {:?}", state);

    print!("> ");
    match io::stdout().flush() {
        _result => {
            for line in io::stdin().lock().lines() {
                match line.unwrap().as_str() {
                    "push" => state = next_state(state, Event::Push),
                    "coin" => state = next_state(state, Event::Coin),
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