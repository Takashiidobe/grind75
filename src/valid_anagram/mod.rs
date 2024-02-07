use std::collections::HashMap;

pub fn is_anagram(s: &str, t: &str) -> bool {
    let mut s_map: HashMap<char, u32> = HashMap::new();
    let mut t_map: HashMap<char, u32> = HashMap::new();

    s.chars().for_each(|c| *s_map.entry(c).or_default() += 1);
    t.chars().for_each(|c| *t_map.entry(c).or_default() += 1);

    s_map == t_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(is_anagram("", ""));
    }

    #[test]
    fn true_example() {
        assert!(is_anagram("anagram", "nagaram"));
    }

    #[test]
    fn false_example() {
        assert!(!is_anagram("rat", "car"));
    }

    #[test]
    fn unicode_true() {
        assert!(is_anagram("日本", "本日"));
    }

    #[test]
    fn unicode_false() {
        assert!(!is_anagram("韓国", "日本"));
    }
}
