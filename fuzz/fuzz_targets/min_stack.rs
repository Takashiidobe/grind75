#![no_main]

use libfuzzer_sys::{arbitrary, fuzz_target};

use grind75::min_stack::MinStack;

#[derive(Debug, arbitrary::Arbitrary)]
enum Command {
    Push(i32),
    Pop,
    Top,
    GetMin,
}

fuzz_target!(|data: Vec<Command>| {
    let mut stack = MinStack::default();
    for command in data {
        match command {
            Command::Push(num) => stack.push(num),
            Command::Pop => stack.pop(),
            Command::Top => {
                stack.top();
            }
            Command::GetMin => {
                stack.get_min();
            }
        }
    }
});
