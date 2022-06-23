use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/odd-even-linked-list/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/odd-even-linked-list/discuss/1008405/Rust-onepass
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head_odd = ListNode::new(-1);
        let mut dummy_head_even = ListNode::new(-1);
        let mut cur_odd = &mut dummy_head_odd;
        let mut cur_even = &mut dummy_head_even;
        let mut is_even = false;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if is_even {
                cur_even.next = Some(node);
                cur_even = cur_even.next.as_mut().unwrap();
            } else {
                cur_odd.next = Some(node);
                cur_odd = cur_odd.next.as_mut().unwrap();
            }
            is_even = !is_even;
        }
        cur_odd.next = dummy_head_even.next;
        dummy_head_odd.next
    }
}
