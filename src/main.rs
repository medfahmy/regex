mod turnstile;

type FsmIndex = usize;

struct FsmColumn {
    ts: [FsmIndex; 127],
}


impl FsmColumn {
    fn new() -> Self {
        FsmColumn { ts: [0; 127] }
    }

    fn f(range: Range<usize>, state: FsmIndex) {

    }
}

type Fsm = Vec<FsmColumn>;

fn main() {
    let mut fsm = Fsm::new();
    fsm.push(FsmColumn::new());

    // turnstile::play();
}
