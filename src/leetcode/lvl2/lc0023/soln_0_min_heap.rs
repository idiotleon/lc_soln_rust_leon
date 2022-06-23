use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/merge-k-sorted-lists/
/// Time Complexity:    O(`_len_ls` * lg(`_len_ls`))
/// Space Complexity:   O(`_len_ls`)
/// Reference:
/// https://leetcode.com/problems/merge-k-sorted-lists/discuss/659745/Rust-BinaryHeap-solution
/// https://leetcode.com/problems/merge-k-sorted-lists/discuss/420969/Rust-Min-Heap
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let _len_ls: usize = lists.len();
        if lists.is_empty() {
            return None;
        }
        let mut heap: BinaryHeap<Reverse<Box<ListNode>>> = BinaryHeap::new();
        for list in lists {
            if let Some(l) = list {
                heap.push(Reverse(l));
            }
        }
        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy;
        while let Some(Reverse(top)) = heap.pop() {
            cur.next = Some(Box::new(ListNode::new(top.val)));
            cur = cur.next.as_mut().unwrap();
            if let Some(next) = top.next {
                heap.push(Reverse(next));
            }
        }
        dummy.next
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
