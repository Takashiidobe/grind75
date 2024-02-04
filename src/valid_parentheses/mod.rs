use std::collections::HashMap;

pub fn is_valid(s: &str) -> bool {
    let mut stack = vec![];
    let hm = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    for c in s.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                if stack.last() == Some(hm.get(&c).unwrap()) {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(is_valid(""));
    }

    #[test]
    fn parens() {
        assert!(is_valid("()"));
    }

    #[test]
    fn wrong_order() {
        assert!(!is_valid(")("));
    }
}
