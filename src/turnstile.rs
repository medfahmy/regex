
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