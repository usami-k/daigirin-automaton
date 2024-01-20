type Transitions = Vec<(char, usize)>;

struct StateMachine {
    transitions: Vec<Transitions>,
    initial_state: usize,
    final_state: usize,
}

impl StateMachine {
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
        current_state == self.final_state
    }
}

fn create_state_machine() -> StateMachine {
    StateMachine {
        transitions: vec![
            vec![('a', 1)],
            vec![('b', 2)],
            vec![('a', 2)],
        ],
        initial_state: 0,
        final_state: 2,
    }
}

// fn create_state_machine() -> StateMachine {
//     StateMachine {
//         transitions: vec![
//             vec![('a', 1)],
//             vec![('b', 2), ('c', 2)],
//             vec![('b', 2), ('c', 2), ('d', 3)],
//             vec![],
//         ],
//         initial_state: 0,
//         final_state: 3,
//     }
// }

fn main() {
    let state_machine = create_state_machine();
    let text = "abaa";
    let matched = state_machine.accept(text);
    if matched {
        println!("Matched!");
    } else {
        println!("Not matched!");
    }
}
