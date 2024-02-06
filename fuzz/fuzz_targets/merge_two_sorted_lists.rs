#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::merge_two_sorted_lists::merge_two_lists;
use grind75::utils::ListNode;

fuzz_target!(|data: (Vec<i32>, Vec<i32>)| {
    let (mut left_vec, mut right_vec) = data;
    left_vec.sort();
    right_vec.sort();

    let left = ListNode::from_vec(left_vec.clone());
    let right = ListNode::from_vec(right_vec.clone());

    let mut sorted_vec = left_vec.clone();
    sorted_vec.extend_from_slice(&right_vec);
    sorted_vec.sort();

    let res_list = ListNode::from_vec(sorted_vec);
    assert_eq!(merge_two_lists(left, right), res_list);
});
