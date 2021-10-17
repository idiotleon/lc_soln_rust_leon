use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use super::tree_node::TreeNode;

pub struct BinaryTree;

#[allow(dead_code)]
impl BinaryTree {
    pub fn new(nums_op: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let len_n: usize = nums_op.len();
        if nums_op.is_empty() || nums_op[0] == None {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums_op[0].unwrap())));
        let mut idx: usize = 1;
        let mut lvl: u32 = 1;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.clone());
        'outer: while !queue.is_empty() {
            let len_lvl: i32 = (2 as i32).pow(lvl);
            for _ in 0..len_lvl {
                if idx == len_n {
                    break 'outer;
                }
                let cur = queue.pop_front().unwrap();
                match nums_op[idx] {
                    Some(num) => {
                        let left = Rc::new(RefCell::new(TreeNode::new(num)));
                        cur.borrow_mut().left = Some(left.clone());
                        queue.push_back(left.clone());
                    }
                    None => cur.borrow_mut().left = None,
                }
                idx += 1;
                if idx == len_n {
                    break 'outer;
                }
                match nums_op[idx] {
                    Some(num) => {
                        let right = Rc::new(RefCell::new(TreeNode::new(num)));
                        cur.borrow_mut().right = Some(right.clone());
                        queue.push_back(right.clone());
                    }
                    None => cur.borrow_mut().right = None,
                }
                idx += 1;
            }
            lvl += 1;
        }
        Some(root)
    }
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if let Some(node) = root {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(node);
            while !queue.is_empty() {
                let len_q = queue.len();
                for _ in 0..len_q {
                    if let Some(cur) = queue.pop_front() {
                        ans.push(cur.borrow().val);
                        if let Some(left) = cur.borrow().left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = cur.borrow().right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input_1_should_work() {
        let nums_op: Vec<Option<i32>> =
            vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)];
        let root = BinaryTree::new(&nums_op);
        let actual: Vec<i32> = BinaryTree::bfs(root);
        let expected: Vec<i32> = vec![2, 2, 5, 5, 7];
        assert_eq!(expected, actual);
    }
}
