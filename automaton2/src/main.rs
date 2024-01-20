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
                if ch == 'b' || ch == 'c' {
                    current_state = 2;
                } else {
                    return false;
                }
            }
            2 => {
                if ch == 'b' || ch == 'c' {
                    current_state = 2;
                } else if ch == 'd' {
                    current_state = 3;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    current_state == 3
}

fn main() {
    let text = "abcbd";
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
        assert!(accept("abd"));
        assert!(accept("acd"));
        assert!(accept("abcbcd"));
        assert!(accept("acccd"));
        assert!(accept("abbbbbd"));
    }

    #[test]
    fn test_accept_failure() {
        assert!(!accept("a"));
        assert!(!accept("abc"));
        assert!(!accept("ad"));
        assert!(!accept("abcdef"));
        assert!(!accept("xyz"));
    }
}
