// https://leetcode.com/problems/deepest-leaves-sum/
//
// Time Complexity:     O(N)
// Space Complexity:    O(W)
//
// Reference:
//  https://leetcode.com/problems/deepest-leaves-sum/discuss/1153427/Rust-BFS-solution
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        
        let mut sum = 0;
        while !queue.is_empty(){
            sum = 0;
            
            for _ in 0..queue.len(){
                if let Some(node) = queue.pop_front(){
                    sum += node.borrow().val;
                    
                    if let Some(left) = node.borrow_mut().left.take(){
                        queue.push_back(left);
                    }
                    
                    if let Some(right) = node.borrow_mut().right.take(){
                        queue.push_back(right);
                    }
                }
            }
        }
        
        sum
    }
}