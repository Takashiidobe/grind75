#![no_main]

use std::collections::HashSet;

use libfuzzer_sys::fuzz_target;

use grind75::binary_search::search;

fuzz_target!(|data: (Vec<i32>, i32)| {
    let (mut items, target) = data;
    items.sort();
    let res = search(&items, target);
    let set: HashSet<i32> = HashSet::from_iter(items.into_iter());

    if set.contains(&target) {
        assert!(res.is_ok());
    } else {
        assert!(res.is_err());
    }
});
