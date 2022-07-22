use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/partition-list/
/// Time Complexity:     O(L)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/partition-list/discuss/719013/Rust-100-fast-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // dummy head for the lo
        let mut dummy_lo = Some(Box::new(ListNode::new(0)));
        // (prev) cursor for the lo
        let mut prev_lo = dummy_lo.as_mut();
        // dummy head for the hi
        let mut dummy_hi = Some(Box::new(ListNode::new(0)));
        // (prev) cursor for the hi
        let mut prev_hi = dummy_hi.as_mut();
        let mut curr = head;
        while let Some(node) = curr {
            let value = node.val;
            if value < x {
                if let Some(n) = prev_lo {
                    n.next = Some(node.clone());
                    prev_lo = n.next.as_mut();
                }
            } else {
                if let Some(n) = prev_hi {
                    n.next = Some(node.clone());
                    prev_hi = n.next.as_mut();
                }
            }
            curr = node.next;
        }
        if let Some(n) = prev_hi {
            n.next = None;
        }
        if let Some(n) = prev_lo {
            n.next = dummy_hi.unwrap().next;
        }
        dummy_lo.unwrap().next
    }
}
