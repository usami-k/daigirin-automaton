fn accept(text: &str) -> bool {
    let mut current_state = 0;
    for ch in text.chars() {
        match current_state {
            0 => {
                if ch == 'a' {
                    current_state = 1;
                } else {
                    return false;
                }
            }
            1 => {
                if ch == 'b' {
                    current_state = 2;
                } else {
                    return false;
                }
            }
            2 => {
                if ch == 'a' {
                    current_state = 2;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    current_state == 2
}

fn main() {
    let text = "abaaa";
    let matched = accept(text);
    if matched {
        println!("Matched!");
    } else {
        println!("Not matched!");
    }
}

#[cfg(test)]
mod tests {
    use super::accept;

    #[test]
    fn test_accept_success() {
        assert!(accept("ab"));
        assert!(accept("aba"));
        assert!(accept("abaa"));
        assert!(accept("abaaaa"));
    }

    #[test]
    fn test_accept_failure() {
        assert!(!accept("a"));
        assert!(!accept("b"));
        assert!(!accept("ba"));
        assert!(!accept("abc"));
        assert!(!accept("abac"));
        assert!(!accept("xyz"));
    }
}
