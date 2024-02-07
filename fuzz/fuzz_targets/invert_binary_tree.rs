#![no_main]

use std::{cell::RefCell, rc::Rc};

use libfuzzer_sys::fuzz_target;

use grind75::{invert_binary_tree::invert_tree, utils::TreeNode};

fuzz_target!(|data: Option<Rc<RefCell<TreeNode<i32>>>>| {
    let _ = invert_tree(data);
});
