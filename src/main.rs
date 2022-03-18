#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

type FsmIndex = usize;

const FSM_COL_SIZE: FsmIndex = 130;
const FSM_EOL: FsmIndex = 129;

#[derive(Default, Clone, Copy)]
struct FsmAction {
    next: FsmIndex,
    offset: i32,
}

#[derive(Clone)]
struct FsmCol {
    ts: [FsmAction; FSM_COL_SIZE],
}

impl FsmCol {
    fn new() -> Self {
        FsmCol {
            ts: [Default::default(); FSM_COL_SIZE],
        }
    }
}

struct Regex {
    cols: Vec<FsmCol>,
}

impl Regex {
    fn dump(&self) {
        for symbol in 0..FSM_COL_SIZE {
            print!("{} => ", symbol);
            for col in &self.cols {
                print!("({}, {}) ", col.ts[symbol].next, col.ts[symbol].offset);
            }
            println!();
        }
    }

    fn compile(src: &str) -> Self {
        let mut regex = Self { cols: Vec::new() };

        // failed state
        regex.cols.push(FsmCol::new());

        // FsmColumn 1
        for char in src.chars() {
            let mut col = FsmCol::new();

            match char {
                '$' => {
                    col.ts[FSM_EOL] = FsmAction {
                        next: regex.cols.len() + 1,
                        offset: 1,
                    };
                    regex.cols.push(col);
                }
                '.' => {
                    for i in 32..129 {
                        col.ts[i] = FsmAction {
                            next: regex.cols.len() + 1,
                            offset: 1,
                        };
                    }
                    regex.cols.push(col);
                }
                '*' => {
                    let len = regex.cols.len();
                    // col.ts = fsm.cols[fsm.cols.len() - 1].ts;
                    for t in regex.cols.last_mut().unwrap().ts.iter_mut() {
                        if t.next == len {
                            t.next = len - 1;
                        } else if t.next == 0 {
                            t.next = len;
                            t.offset = 0;
                        } else {
                            unreachable!()
                        };
                    }
                }
                '+' => {
                    regex.cols.push(regex.cols.last().unwrap().clone());

                    let len = regex.cols.len();
                    // col.ts = fsm.cols[fsm.cols.len() - 1].ts;
                    for t in regex.cols.last_mut().unwrap().ts.iter_mut() {
                        if t.next == len {
                            // t.next = len - 1;
                        } else if t.next == 0 {
                            t.next = len + 1;
                            t.offset = 0;
                        } else {
                            unreachable!()
                        };
                    }
                }
                // TODO: '[0-9]'
                _ => {
                    col.ts[char as usize] = FsmAction {
                        next: regex.cols.len() + 1,
                        offset: 1,
                    };
                    regex.cols.push(col);
                }
            }
        }

        regex
    }

    fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let len = chars.len();

        while 0 < state && state < self.cols.len() && head < len {
            let action = self.cols[state].ts[chars[head] as usize];

            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }

        if state == 0 {
            return false;
        } 
        
        if state < self.cols.len() {
            let action = self.cols[state].ts[FSM_EOL];

            state = action.next;
        }

        return state >= self.cols.len();
    }
}

fn main() {
    let src = "a+bc$";
    let mut regex = Regex::compile(src);
    // regex.dump();

    println!("{}", "-".repeat(20));

    println!("Regex: {}", src);

    let inputs = ["bc", "abc", "aaabcd", "aaccd", "abcd"];

    for input in inputs {
        println!("{} => {:?}", input, regex.match_str(input));
    }
}
