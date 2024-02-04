use std::collections::HashSet;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut hm = HashSet::new();

    for left in &nums {
        if let Some(complement) = target.checked_sub(*left) {
            if hm.contains(&complement) {
                return Some((*left, complement));
            } else {
                hm.insert(left);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = vec![];
        assert_eq!(None, two_sum(input, 0));
    }

    #[test]
    fn test_no_match() {
        let input = vec![1, 2, 3];
        assert_eq!(None, two_sum(input, 0));
    }

    #[test]
    fn test_negative_and_positive() {
        let input = vec![1, 2, 3, 4, -5, -3];
        assert_eq!(Some((-3, 3)), two_sum(input, 0));
    }
}
