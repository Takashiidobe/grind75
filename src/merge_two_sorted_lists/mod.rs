use crate::utils::*;

pub fn merge_two_lists(
    list1: Option<Box<ListNode<i32>>>,
    list2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) | (None, Some(l)) => Some(l),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                l1.next = merge_two_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(l2.next, Some(l1));
                Some(l2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slist;

    #[test]
    fn normal() {
        let l1 = slist!(1, 2, 4);
        let l2 = slist!(1, 3, 4);

        assert_eq!(merge_two_lists(l1, l2), slist!(1, 1, 2, 3, 4, 4));
    }

    #[test]
    fn empty() {
        let l1 = slist!();
        let l2 = slist!();

        assert_eq!(merge_two_lists(l1, l2), slist!());
    }

    #[test]
    fn one_side() {
        let l1 = slist!();
        let l2 = slist!(0);

        assert_eq!(merge_two_lists(l1, l2), slist!(0));
    }

    #[test]
    fn other_side() {
        let l1 = slist!(0);
        let l2 = slist!();

        assert_eq!(merge_two_lists(l1, l2), slist!(0));
    }
    #[test]
    fn other_side_longer() {
        let l1 = slist!(0, 1, 2, 3);
        let l2 = slist!();

        assert_eq!(merge_two_lists(l1, l2), slist!(0, 1, 2, 3));
    }
}
