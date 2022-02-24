use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
/// @author: Leon
/// https://leetcode.com/problems/sort-list/
/// Time Complexity:    O(L * lg(L))
/// Space Complexity:   O(L)
/// Reference:
/// https://leetcode.com/problems/sort-list/discuss/894505/Rust-Simple-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let cur = &mut head;
        let mut heap = BinaryHeap::new();
        while let Some(node) = cur {
            heap.push(Reverse(node.val));
            *cur = node.next.take();
        }
        let mut head: Option<Box<ListNode>> = None;
        let mut cur: &mut Option<Box<ListNode>> = &mut None;
        while !heap.is_empty() {
            let new_node = Some(Box::new(ListNode::new(heap.pop().unwrap().0)));
            if let Some(c) = cur {
                c.next = new_node;
                cur = &mut c.next;
            } else {
                head = new_node;
                cur = &mut head;
            }
        }
        head
    }
}