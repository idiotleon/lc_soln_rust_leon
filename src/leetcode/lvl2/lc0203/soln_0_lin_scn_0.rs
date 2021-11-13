/// https://leetcode.com/problems/remove-linked-list-elements/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-linked-list-elements/discuss/746297/Rust-no-unwraps.
#[allow(dead_code)]
struct Solution;

use crate::leetcode::util::data_structure::list_node::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;
        loop {
            match ptr {
                Some(node) if node.val == val => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
                None => break,
            }
        }
        head
    }
}
