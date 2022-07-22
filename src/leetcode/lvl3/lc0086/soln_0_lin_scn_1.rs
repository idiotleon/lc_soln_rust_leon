use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/partition-list/
/// Time Complexity:    O(L)
/// Space Compleixty:   O(1)
/// Reference:
/// https://leetcode.com/problems/partition-list/discuss/1492448/rust-0ms-clean
/// https://leetcode.com/problems/partition-list/discuss/1520165/clean-rust-code-similar-to-neetcode's-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let (mut lo_head, mut hi_head): (Option<Box<ListNode>>, Option<Box<ListNode>>) =
            (None, None);
        let (mut lo, mut hi) = (&mut lo_head, &mut hi_head);
        let mut cur: Option<Box<ListNode>> = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            if node.val < x {
                *lo = Some(node);
                lo = &mut lo.as_deref_mut().unwrap().next;
            } else {
                *hi = Some(node);
                hi = &mut hi.as_deref_mut().unwrap().next;
            }
        }
        *lo = hi_head;
        lo_head
    }
}
