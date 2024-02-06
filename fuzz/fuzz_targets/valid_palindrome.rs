#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::valid_palindrome::valid_palindrome;

fuzz_target!(|data: &str| {
    let _ = valid_palindrome(data);
});
