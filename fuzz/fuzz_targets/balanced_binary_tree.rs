#![no_main]

use std::{cell::RefCell, rc::Rc};

use libfuzzer_sys::fuzz_target;

use grind75::{balanced_binary_tree::is_balanced, utils::TreeNode};

fuzz_target!(|data: Option<Rc<RefCell<TreeNode<i32>>>>| {
    let _ = is_balanced(data);
});
