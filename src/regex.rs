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
    tokens: [FsmAction; FSM_COL_SIZE],
}

impl FsmCol {
    fn new() -> Self {
        FsmCol {
            tokens: [Default::default(); FSM_COL_SIZE],
        }
    }
}

pub struct Regex {
    columns: Vec<FsmCol>,
}

impl Regex {
    pub fn dump(&self) {
        for symbol in 0..FSM_COL_SIZE {
            print!("{} => ", symbol);
            for col in &self.columns {
                print!("({}, {}) ", col.tokens[symbol].next, col.tokens[symbol].offset);
            }
            println!();
        }
    }

    pub fn compile(src: &str) -> Self {
        let mut regex = Self { columns: Vec::new() };

        // failed state
        regex.columns.push(FsmCol::new());

        // FsmColumn 1
        for char in src.chars() {
            let mut col = FsmCol::new();

            match char {
                '$' => {
                    col.tokens[FSM_EOL] = FsmAction {
                        next: regex.columns.len() + 1,
                        offset: 1,
                    };
                    regex.columns.push(col);
                }
                '.' => {
                    for i in 32..129 {
                        col.tokens[i] = FsmAction {
                            next: regex.columns.len() + 1,
                            offset: 1,
                        };
                    }
                    regex.columns.push(col);
                }
                '*' => {
                    let len = regex.columns.len();
                    // col.ts = fsm.cols[fsm.cols.len() - 1].ts;
                    for t in regex.columns.last_mut().unwrap().tokens.iter_mut() {
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
                    regex.columns.push(regex.columns.last().unwrap().clone());

                    let len = regex.columns.len();
                    // col.ts = fsm.cols[fsm.cols.len() - 1].ts;
                    for t in regex.columns.last_mut().unwrap().tokens.iter_mut() {
                        if t.next == len {
                            // t.next = len - 1;
                        } else if t.next == 0 {
                            t.next = len + 1;
                            t.offset = 0;
                        } else {
                            unreachable!()
                        };
                    }

                    // FIXME: panic!, unreachable code??
                }
                // TODO: '[a-d]'
                // TODO: '{1,9}'
                _ => {
                    col.tokens[char as usize] = FsmAction {
                        next: regex.columns.len() + 1,
                        offset: 1,
                    };
                    regex.columns.push(col);
                }
            }
        }

        regex
    }

    pub fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let len = chars.len();

        while 0 < state && state < self.columns.len() && head < len {
            let action = self.columns[state].tokens[chars[head] as usize];

            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }

        if state == 0 {
            return false;
        } 
        
        if state < self.columns.len() {
            let action = self.columns[state].tokens[FSM_EOL];

            state = action.next;
        }

        return state >= self.columns.len();
    }
}

