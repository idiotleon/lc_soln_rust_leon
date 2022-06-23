use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/path-sum-iii/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/path-sum-iii/discuss/1317371/Rust-Recursive-with-Prefix-Sum
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut sum_to_freq = {
            let mut tmp = HashMap::<i32, i32>::new();
            tmp.insert(0, 1);
            tmp
        };
        Self::backtrack(0, target_sum, root, &mut sum_to_freq)
    }
    fn backtrack(
        cur_sum: i32,
        target_sum: i32,
        node: Option<Rc<RefCell<TreeNode>>>,
        sum_to_freq: &mut HashMap<i32, i32>,
    ) -> i32 {
        match node {
            Some(cur_node) => {
                let mut freq = 0;
                let next_sum = cur_sum + cur_node.borrow().val;
                freq += sum_to_freq.get(&(next_sum - target_sum)).unwrap_or(&0);
                sum_to_freq.entry(next_sum).or_insert(0);
                *sum_to_freq.get_mut(&next_sum).unwrap() += 1;
                freq += Self::backtrack(
                    next_sum,
                    target_sum,
                    cur_node.borrow().left.clone(),
                    sum_to_freq,
                );
                freq += Self::backtrack(
                    next_sum,
                    target_sum,
                    cur_node.borrow().right.clone(),
                    sum_to_freq,
                );
                *sum_to_freq.get_mut(&next_sum).unwrap() -= 1;
                freq
            }
            None => 0,
        }
    }
}
