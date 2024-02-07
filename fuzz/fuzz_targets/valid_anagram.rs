#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::valid_anagram::is_anagram;

fuzz_target!(|data: (&str, &str)| {
    let (left_str, right_str) = data;
    let mut left_chars: Vec<_> = left_str.chars().collect();
    let mut right_chars: Vec<_> = right_str.chars().collect();

    left_chars.sort();
    right_chars.sort();

    assert_eq!(left_chars == right_chars, is_anagram(left_str, right_str));
});
