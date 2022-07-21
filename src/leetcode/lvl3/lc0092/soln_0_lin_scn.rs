use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/reverse-linked-list-ii/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/reverse-linked-list-ii/discuss/2310919/Rust-%3A-Safe-O(1)-space-but-two-pass
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut prev: &mut Option<Box<ListNode>> = &mut dummy;
        for _ in 1..left {
            prev = &mut prev.as_mut().unwrap().next;
        }
        let mut cur: Option<Box<ListNode>> = prev.as_mut().unwrap().next.take();
        let mut next: Option<Box<ListNode>> = cur.as_mut().unwrap().next.take();
        for _ in left..right {
            let next_next: Option<Box<ListNode>> = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = cur;
            cur = next;
            next = next_next;
        }
        let mut tail_rev: &mut Option<Box<ListNode>> = &mut cur;
        for _ in left..right {
            tail_rev = &mut tail_rev.as_mut().unwrap().next;
        }
        tail_rev.as_mut().unwrap().next = next;
        prev.as_mut().unwrap().next = cur;
        dummy.unwrap().next
    }
}
