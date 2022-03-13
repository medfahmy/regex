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
        RegexCol {
            ts: [0; REGEX_COL_SIZE],
        }
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
    fn compile_regex(src: &str) -> Self {
        /*
        let events = ['a' as usize, 'b' as usize, 'c' as usize, REGEX_END];

        // failed state
        regex.push(RegexCol::new());

        // FsmColumn 1
        for event in events.iter() {
            let mut col = RegexCol::new();
            col.ts[*event] = regex.cols.len() + 1;
            regex.push(col);
        }
        */

        todo!()
    }

    fn match_regex(&self, input: &str) -> bool {
        let mut state = 1;
        for c in input.chars() {
            if state == 0 || state >= self.cols.len() {
                break;
            }

            state = self.cols[state].ts[c as usize];
        }

        if state == 0 {
            return false;
        } else if state < self.cols.len() {
            state = self.cols[state].ts[REGEX_END];
        }

        return state >= self.cols.len();
    }
}

fn main() {
    let mut regex = Regex::compile_regex("abc$");

    // regex.dump();

    let inputs = ["hello world", "abc", "abcd"];

    for input in inputs {
        println!("{} => {:?}", input, regex.match_regex(input));
    }

    // let mut col = FsmCol::new();

    // col.fill_range();
    // fsm.push();

    // turnstile::play();

    // println!("{:?}", ('a'..'z').map(|x| x as u8).collect::<Vec<u8>>());

    // println!("{:?}", vec!['a', 'b', 'c'].into_iter().map(|c| c as u8).rev().collect::<Vec<u8>>());
}
