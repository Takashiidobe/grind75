#![no_main]

use libfuzzer_sys::fuzz_target;

use grind75::best_time_to_buy_and_sell_stock::max_profit;

fuzz_target!(|data: Vec<u32>| {
    if data.len() < 2 {
        return;
    }
    let mut copy = data.clone();
    let min_val = data.iter().min().unwrap();
    copy.push(u32::MAX);
    assert_eq!(max_profit(&copy), u32::MAX - min_val);
});
