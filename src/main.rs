use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, Copy)]
enum State {
    Locked,
    Unlocked,
}

#[derive(Debug)]
enum Event {
    Push,
    Coin,
}

use State::*;
use Event::*;

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

fn next_state(state: State, event: Event) -> State {
    FSM[state as usize][event as usize]
}


fn main() {
    let mut state = Locked;

    // let arr = [1, 2, 4];
    // println!("{}", arr[state as usize]);

    println!("State: {:?}", state);

    print!("> ");
    io::stdout().flush();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "push" => state = next_state(state, Push),
            "coin" => state = next_state(state, Coin),
            "quit" => return,
            unknown => eprintln!("ERROR: unknown event {}", unknown)
        }

        println!("{:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
