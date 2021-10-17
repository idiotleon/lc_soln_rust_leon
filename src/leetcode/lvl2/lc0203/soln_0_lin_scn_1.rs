use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// https://leetcode.com/problems/remove-linked-list-elements/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference;
/// https://leetcode.com/problems/remove-linked-list-elements/discuss/1008395/Rust-onepass
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode {
            val: -1,
            next: None,
        };
        let mut cur = &mut dummy_head;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if node.val != val {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}
