#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{ops::Range, usize};

mod turnstile;

type RegexIndex = usize;

const REGEX_COL_SIZE: RegexIndex = 130; 
const REGEX_END: RegexIndex = 129; 

struct RegexCol {
    ts: [RegexIndex; REGEX_COL_SIZE],
}

impl RegexCol {
    fn new() -> Self {
        RegexCol { ts: [0; REGEX_COL_SIZE] }
    }

    fn fill_range(&mut self, range: Range<char>, state: RegexIndex) {
        for i in range.map(|x| x as usize) {
            self.ts[i] = state;
        }
    }
}

struct Regex {
    cols: Vec<RegexCol>,
} 

impl Regex {
    fn new() -> Self {
        Self { cols: Vec::new() }
    }

    fn push(&mut self, col: RegexCol) {
        self.cols.push(col);
    }

    fn dump(&self) {
        for symbol in 0..REGEX_COL_SIZE {
            print!("{} => ", symbol);
            for col in &self.cols {
                print!("{} ", col.ts[symbol]);
            }
            println!();
        }

    }
}

fn compile_regex(regex: &Regex, src: &str) -> Regex {

    todo!()
}

fn match_regex(regex: &Regex, input: &str) -> bool {
    let mut state = 1;
    for c in input.chars() {
        if state == 0 || state >= regex.cols.len() {
            break;
        }

        state = regex.cols[state].ts[c as usize];
    }

    if state == 0 {
        return false;
    } else if state < regex.cols.len() {
        state = regex.cols[state].ts[REGEX_END];
    }

    return state >= regex.cols.len();
}

fn main() {
    let mut fsm = Regex::new();

    let events = ['a' as usize, 'b' as usize, 'c' as usize, REGEX_END];

    // failed state
    fsm.push(RegexCol::new());

    // FsmColumn 1
    for event in events.iter() {
        let mut col = RegexCol::new();
        col.ts[*event] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    // fsm.dump();

    let inputs = ["hello world", "abc", "abcd"];

    for input in inputs {
        println!("{} => {:?}", input, match_regex(&fsm, input));
    }

    // let mut col = FsmCol::new();

    // col.fill_range();
    // fsm.push();

    // turnstile::play();

    // println!("{:?}", ('a'..'z').map(|x| x as u8).collect::<Vec<u8>>());

    // println!("{:?}", vec!['a', 'b', 'c'].into_iter().map(|c| c as u8).rev().collect::<Vec<u8>>());
}
