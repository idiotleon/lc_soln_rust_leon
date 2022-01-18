use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
/// Note:
/// this approach takes the assumption that values of all tree nodes are distinct
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if let Some(node) = root {
            let chd_to_prnt: HashMap<i32, Rc<RefCell<TreeNode>>> = {
                let mut ch_to_prnt: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
                Self::dfs(node.clone(), &mut ch_to_prnt);
                ch_to_prnt
            };
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            let mut seen: HashSet<i32> = HashSet::new();
            if let Some(t) = target {
                queue.push_back(t.clone());
                let value = t.clone().borrow().val;
                seen.insert(value);
                // sanity check, required
                if k == 0 {
                    return vec![value];
                }
            }
            let mut dist: u16 = 0;
            while !queue.is_empty() {
                let len_q: usize = queue.len();
                for _ in 0..len_q {
                    if let Some(cur) = queue.pop_front() {
                        ans.push(cur.borrow().val);
                        if let Some(left) = cur.clone().borrow().left.clone() {
                            if seen.insert(left.clone().borrow().val) {
                                queue.push_back(left);
                            }
                        }
                        if let Some(right) = cur.clone().borrow().right.clone() {
                            if seen.insert(right.clone().borrow().val) {
                                queue.push_back(right);
                            }
                        }
                        if let Some(prnt) = chd_to_prnt.get(&cur.clone().borrow().val) {
                            if seen.insert(prnt.clone().borrow().val) {
                                queue.push_back(prnt.clone());
                            }
                        }
                    }
                }
                if dist == k as u16 {
                    break;
                }
                dist += 1;
                ans.clear();
            }
        }
        ans
    }
    // this takes the assumption that values of all tree nodes are DISTINCT
    fn dfs(node: Rc<RefCell<TreeNode>>, chd_to_prnt: &mut HashMap<i32, Rc<RefCell<TreeNode>>>) {
        if let Some(left) = node.borrow().left.clone() {
            chd_to_prnt.insert(left.borrow().val, node.clone());
            Self::dfs(left.clone(), chd_to_prnt);
        }
        if let Some(right) = node.borrow().right.clone() {
            chd_to_prnt.insert(right.borrow().val, node.clone());
            Self::dfs(right.clone(), chd_to_prnt);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leetcode::util::data_structure::tree::binary::binary_tree::BinaryTree;
    #[test]
    fn it_works_with_test_case_33() {
        let values_tree_node: Vec<Option<i32>> = vec![Some(0), Some(1), None, Some(3), Some(2)];
        let root = BinaryTree::new(&values_tree_node);
        // this approach takes the assumption that values of all tree nodes are distinct,
        // so that the comparision is based on the value of the tree node,
        // instead of the acutal tree node
        let target: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let k: i32 = 1;
        let actual = Solution::distance_k(root, target, k);
        let expected = vec![1];
        assert_eq!(expected, actual);
    }
}
