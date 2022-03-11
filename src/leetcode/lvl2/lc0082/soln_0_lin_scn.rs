use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
/// @author: Leon
/// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/338369/Short-rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        };
        const RANGE: i32 = 100 + 1;
        let mut to_remove: i32 = -RANGE;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: to_remove,
        }));
        let mut node = &mut dummy.as_mut().unwrap().next;
        loop {
            match node {
                None => return dummy.unwrap().next,
                Some(n) if n.val == to_remove => *node = n.next.take(),
                Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                    to_remove = n.val
                }
                Some(n) => {
                    node = &mut n.next;
                    if let Some(n) = node {
                        to_remove = n.val - 1;
                    }
                }
            }
        }
    }
}
