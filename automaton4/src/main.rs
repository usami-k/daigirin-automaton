type State = usize;
type Symbol = char;
type Transitions = Vec<(Symbol, State)>;

struct Dfa {
    states: Vec<State>,
    alphabet: Vec<Symbol>,
    transitions: Vec<Transitions>,
    initial_state: State,
    final_states: Vec<State>,
}

impl Dfa {
    fn accept(&self, text: &str) -> bool {
        let mut current_state = self.initial_state;
        for ch in text.chars() {
            let current_transitions =
                self.transitions.get(current_state).unwrap();
            let transition = current_transitions
                .iter()
                .find(|(symbol, _)| ch == *symbol);
            match transition {
                Some((_, next_state)) => current_state = *next_state,
                None => return false,
            }
        }
        self.final_states.contains(&current_state)
    }
}

fn create_dfa() -> Dfa {
    Dfa {
        states: vec![0, 1, 2],
        alphabet: vec!['a', 'b'],
        transitions: vec![
            vec![('a', 1)],
            vec![('b', 2)],
            vec![('a', 2)],
        ],
        initial_state: 0,
        final_states: vec![2],
    }
}

fn main() {
    let dfa = create_dfa();
    let text = "abaa";
    let matched = dfa.accept(text);
    if matched {
        println!("Matched!");
    } else {
        println!("Not matched!");
    }
}
