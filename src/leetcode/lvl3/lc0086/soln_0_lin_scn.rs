/// https://leetcode.com/problems/partition-list/
///
/// Time Complexity:     O()
/// Space Complexity:    O()
///
/// Reference:
/// https://leetcode.com/problems/partition-list/discuss/719013/Rust-100-fast-solution
use crate::leetcode::util::data_structure::list_node::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // dummy head for the less
        let mut dummy_less = Some(Box::new(ListNode::new(0)));
        // (prev) cursor for the less
        let mut prev_less = dummy_less.as_mut();
        // dummy head for the greater
        let mut dummy_greater = Some(Box::new(ListNode::new(0)));
        // (prev) cursor for the greater
        let mut prev_greater = dummy_greater.as_mut();
        let mut curr = head;
        while let Some(node) = curr {
            if node.val < x {
                if let Some(n) = prev_less {
                    n.next = Some(node.clone());
                    prev_less = n.next.as_mut();
                }
            } else {
                if let Some(n) = prev_greater {
                    n.next = Some(node.clone());
                    prev_greater = n.next.as_mut();
                }
            }
            curr = node.next;
        }
        if let Some(n) = prev_greater {
            n.next = None;
        }
        if let Some(n) = prev_less {
            n.next = dummy_greater.unwrap().next;
        }
        dummy_less.unwrap().next
    }
}
