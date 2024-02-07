use arbitrary::Arbitrary;

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Arbitrary)]
pub struct ListNode<T>
where
    T: Clone,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where
    T: Clone,
{
    pub fn new(val: T) -> ListNode<T> {
        ListNode { next: None, val }
    }

    pub fn box_self(&self) -> Option<Box<Self>> {
        if self.next.is_none() {
            None
        } else {
            Some(Box::new(self.clone()))
        }
    }
}

impl<T> ListNode<T>
where
    T: Default + Clone,
{
    pub fn from_vec(value: Vec<T>) -> Option<Box<ListNode<T>>> {
        if value.is_empty() {
            return None;
        }

        let mut head = ListNode::new(T::default());
        let mut node = &mut head;
        for val in value {
            node.next = Some(Box::new(ListNode::new(val)));
            node = node.next.as_mut().unwrap().as_mut();
        }

        let ListNode { next: result, .. } = head;
        result
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Arbitrary)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Default + Clone,
{
    pub fn from_vec(elems: Vec<T>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if elems.is_empty() {
            return None;
        }

        let head = Some(Rc::new(RefCell::new(TreeNode::new(elems[0].clone()))));
        let mut nodes = std::collections::VecDeque::new();
        nodes.push_back(head.as_ref().unwrap().clone());

        for i in elems[1..].chunks(2) {
            let node = nodes.pop_front().unwrap();
            if let Some(val) = i.first() {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val.clone()))));
                nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
            }
            if i.len() > 1 {
                if let Some(val) = i.get(1) {
                    node.borrow_mut().right =
                        Some(Rc::new(RefCell::new(TreeNode::new(val.clone()))));
                    nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        head
    }
}

#[macro_export]
macro_rules! test {
    ($($name:ident: $left:expr, $right:expr,)*) => {
        #[cfg(test)]
        mod test {
        use super::*;
            $(
                #[test]
                fn $name() {
                    assert_eq!($left, $right);
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            use std::rc::Rc;
            use std::cell::RefCell;

            let elems = vec![$(stringify!($e)), *];
            let elems = elems.iter().map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
            let head = Some(Rc::new(RefCell::new(TreeNode::new(elems[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}

#[macro_export]
macro_rules! slist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new(ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new(ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}

#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}
