pub fn search<T: Ord>(nums: &[T], target: T) -> Result<usize, usize> {
    nums.binary_search(&target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exists() {
        assert_eq!(search(&[-1, 0, 3, 5, 9, 12], 9), Ok(4));
    }

    #[test]
    fn does_not_exist() {
        assert_eq!(search(&[-1, 0, 3, 5, 9, 12], 2), Err(2));
    }
}
