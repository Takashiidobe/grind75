pub fn max_profit(prices: &[u32]) -> u32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut max_so_far = 0;
    let mut prev_min = *prices
        .first()
        .expect("Can't fail because of len check above");
    for price in prices.iter().skip(1) {
        if *price > prev_min {
            max_so_far = u32::max(price - prev_min, max_so_far);
        }
        prev_min = u32::min(*price, prev_min);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(max_profit(&[]), 0);
    }

    #[test]
    fn one_item() {
        assert_eq!(max_profit(&[3]), 0);
    }

    #[test]
    fn example() {
        assert_eq!(max_profit(&[1, 2, 3, 4]), 3);
    }

    #[test]
    fn no_profit() {
        assert_eq!(max_profit(&[4, 3, 2, 1]), 0);
    }
}
