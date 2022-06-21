// use std::collections::VecDeque;

// use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
// /// @author: Leon
/// https://leetcode.com/problems/add-two-numbers/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    // pub fn add_two_numbers(
    //     l1: Option<Box<ListNode>>,
    //     l2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut stk1: VecDeque<i32> = {
    //         let mut stk: VecDeque<i32> = VecDeque::new();
    //         let mut h = &l1;
    //         while let Some(cur) = h {
    //             stk.push_back(cur.val);
    //             h = &cur.next;
    //         }
    //         stk
    //     };
    //     let mut stk2: VecDeque<i32> = {
    //         let mut stk: VecDeque<i32> = VecDeque::new();
    //         let mut h = &l2;
    //         while let Some(cur) = h {
    //             stk.push_back(cur.val);
    //             h = &cur.next;
    //         }
    //         stk
    //     };
    //     let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    //     let mut head = &mut dummy;
    //     let mut carry: i32 = 0;
    //     while !stk1.is_empty() || !stk2.is_empty() {
    //         let mut sum: i32 = 0;
    //         if let Some(top1) = stk1.pop_back() {
    //             sum += top1;
    //         }
    //         if let Some(top2) = stk2.pop_back() {
    //             sum += top2;
    //         }
    //         sum += carry;
    //         let digit = sum / 10;
    //         carry = sum % 10;
    //         if let Some(ref mut cur) = head {
    //             cur.next = Some(Box::new(ListNode::new(digit)));
    //             head = cur.next;
    //         }
    //     }
    //     dummy.unwrap().next
    // }
}
