pub fn valid_palindrome(s: &str) -> bool {
    let mut filtered: Vec<_> = s.chars().filter(|x| x.is_ascii_alphanumeric()).collect();
    let filtered_len = filtered.len();
    let midpoint = filtered_len / 2;
    let (left, right) = filtered.split_at_mut(midpoint);
    right.reverse();

    if filtered_len % 2 == 0 {
        left == right
    } else {
        let mut right_to_return = right.to_owned();
        if !right_to_return.is_empty() {
            right_to_return.pop();
        }

        left == right_to_return
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_len() {
        let input = "race ;car";
        assert!(valid_palindrome(input));
    }

    #[test]
    fn even_len() {
        let input = "rac ;car";
        assert!(valid_palindrome(input));
    }
}
