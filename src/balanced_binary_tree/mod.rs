use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    fn dfs(node: &Option<Rc<RefCell<TreeNode<i32>>>>) -> Option<i32> {
        match node {
            Some(n) => {
                let borrowed = n.as_ref().borrow();
                let left_side = dfs(&borrowed.left);
                let right_side = dfs(&borrowed.right);
                match (left_side, right_side) {
                    (Some(left_height), Some(right_height)) => {
                        let diff = (left_height - right_height).abs();
                        if diff <= 1 {
                            Some(std::cmp::max(left_height, right_height) + 1)
                        } else {
                            None
                        }
                    }
                    (_, _) => None,
                }
            }
            None => Some(0),
        }
    }
    dfs(&root).is_some()
}

#[cfg(test)]
mod tests {
    use crate::btree;

    use super::*;

    #[test]
    fn unbalanced() {
        assert!(!is_balanced(btree![1, 2, 2, 3, 3, null, null, 4, 4]))
    }

    #[test]
    fn balanced() {
        assert!(is_balanced(btree![3, 9, 20, null, null, 15, 7]))
    }

    #[test]
    fn empty() {
        assert!(is_balanced(btree![]))
    }
}
