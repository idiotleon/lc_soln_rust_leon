use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/validate-binary-tree-nodes
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(W) ~ O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn validate_binary_tree_nodes(
        n: i32,
        left_children: Vec<i32>,
        right_children: Vec<i32>,
    ) -> bool {
        let root: i32 = Self::find_root(n, &left_children, &right_children);
        if root == -1 {
            return false;
        }
        let n: usize = n as usize;
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
        queue.push_back(root as usize);
        let mut seen: HashSet<usize> = HashSet::with_capacity(n);
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if seen.insert(cur) {
                        let left = left_children[cur];
                        if left != -1 {
                            queue.push_back(left as usize);
                        }
                        let right = right_children[cur];
                        if right != -1 {
                            queue.push_back(right as usize);
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        return seen.len() == n;
    }
    fn find_root(n: i32, left_children: &Vec<i32>, right_children: &Vec<i32>) -> i32 {
        let all_children: HashSet<i32> = {
            let mut all: HashSet<i32> = HashSet::with_capacity(n as usize);
            for &child in left_children {
                if child != -1 {
                    all.insert(child);
                }
            }
            for &child in right_children {
                if child != -1 {
                    all.insert(child);
                }
            }
            all
        };
        for idx in 0..n {
            if !all_children.contains(&idx) {
                return idx;
            }
        }
        return -1;
    }
}
