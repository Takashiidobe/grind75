use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn invert_tree<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
    if let Some(ref n) = root {
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();

        let mut n = n.borrow_mut();
        n.left = invert_tree(right);
        n.right = invert_tree(left);
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btree;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let tree: Option<Rc<RefCell<TreeNode<i32>>>> = btree!();
        assert_eq!(invert_tree(tree), btree!());
    }

    #[test]
    fn it_works() {
        let tree = btree!(1, 2, 3);
        assert_eq!(invert_tree(tree), btree!(1, 3, 2));
    }
}
