#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::two_sum::two_sum;

fuzz_target!(|data: (Vec<i32>, i32)| {
    let (nums, target) = data;
    if let Some((left, right)) = two_sum(nums.clone(), target) {
        assert_eq!(left + right, target);
    }
});
