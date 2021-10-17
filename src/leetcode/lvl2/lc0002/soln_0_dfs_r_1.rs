/// https://leetcode.com/problems/add-two-numbers/
///
/// Time Complexity:    O(len1 + len2) ~ O(max(len1, len2))
/// Space ComplexitY;   O(len1 + len2) ~ O(max(len1, len2)))
///
/// Reference:
/// https://leetcode.com/problems/add-two-numbers/discuss/261694/Rust-4ms
use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::merge(l1, l2, 0, ListNode::new(-1))
    }

    fn merge(
        mut cur1: Option<Box<ListNode>>,
        mut cur2: Option<Box<ListNode>>,
        mut sum: i32,
        mut new_node: ListNode,
    ) -> Option<Box<ListNode>> {
        if cur1.is_none() && cur2.is_none() && sum == 0 {
            return None;
        }

        if let Some(n1) = cur1 {
            sum += n1.val;
            cur1 = n1.next;
        }

        if let Some(n2) = cur2 {
            sum += n2.val;
            cur2 = n2.next;
        }

        new_node.val = if sum > 9 { sum - 10 } else { sum };
        new_node.next = Self::merge(cur1, cur2, sum / 10, ListNode::new(-1));

        Some(Box::new(new_node))
    }
}
