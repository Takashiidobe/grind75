#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::valid_parentheses::is_valid;

fuzz_target!(|data: &str| {
    let _ = is_valid(data);
});
