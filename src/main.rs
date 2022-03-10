#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::ops::Range;

mod turnstile;

type FsmIndex = usize;

struct FsmColumn {
    ts: [FsmIndex; 127],
}

impl FsmColumn {
    fn new() -> Self {
        FsmColumn { ts: [0; 127] }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
        for i in range.map(|x| x as usize) {
            self.ts[i] = state;
        }
    }
}

type Fsm = Vec<FsmColumn>;

fn main() {
    let mut fsm = Fsm::new();

    let mut col = FsmColumn::new();

    // col.fill_range();
    // fsm.push();

    // turnstile::play();

    println!("{:?}", ('a'..'z').map(|x| x as u8).collect::<Vec<u8>>());

    // println!("{:?}", vec!['a', 'b', 'c'].into_iter().map(|c| c as u8).rev().collect::<Vec<u8>>());
}
