#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

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
}

struct Regex {
    cols: Vec<RegexCol>,
}

impl Regex {
    fn dump(&self) {
        for symbol in 0..REGEX_COL_SIZE {
            print!("{} => ", symbol);
            for col in &self.cols {
                print!("{} ", col.ts[symbol]);
            }
            println!();
        }
    }

    fn compile(src: &str) -> Self {
        let mut result = Self { cols: Vec::new() };

        // failed state
        result.cols.push(RegexCol::new());

        // FsmColumn 1
        for char in src.chars() {
            let mut col = RegexCol::new();

            match char {
                '$' => {
                    col.ts[REGEX_END] = result.cols.len() + 1;
                }
                '.' => {
                    for i in 32..129 {
                        col.ts[i] = result.cols.len() + 1;
                    }
                }
                _ => {
                    col.ts[char as usize] = result.cols.len() + 1;
                }
            }

            result.cols.push(col);
        }

        result
    }

    fn match_str(&self, input: &str) -> bool {
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
    let mut regex = Regex::compile("a.cd$");
    regex.dump();

    println!("{}", "-".repeat(20));

    let inputs = ["hello world", "abc", "abcd"];

    for input in inputs {
        println!("{} => {:?}", input, regex.match_str(input));
    }
}
