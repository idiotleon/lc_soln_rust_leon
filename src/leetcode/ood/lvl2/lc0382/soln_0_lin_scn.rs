use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
use rand::prelude::*;
/// https://leetcode.com/problems/linked-list-random-node/
/// Time Complexities:
///     `new()`:        O(L)
///     `get_random()`: O(1)
/// Space Complexity:   O(L)
struct Solution {
    len: usize,
    list: Option<Box<ListNode>>,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut cur = head.as_ref();
        let mut len = 0;
        while let Some(node) = cur {
            cur = node.next.as_ref();
            len += 1;
        }
        Self { len, list: head }
    }

    fn get_random(&self) -> i32 {
        if self.len == 0 {
            return -1;
        }
        if self.len == 1 {
            return self.list.as_ref().unwrap().val;
        }
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(0..self.len);
        let mut cur = 0;
        let mut node = self.list.as_ref().unwrap();
        while cur < target {
            node = node.next.as_ref().unwrap();
            cur += 1;
        }
        node.val
    }
}
