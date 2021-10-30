use std::cell::RefCell;
/// https://leetcode.com/problems/binary-tree-cameras/
///
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
///
/// Reference:
/// https://leetcode.com/problems/binary-tree-cameras/discuss/211180/JavaC%2B%2BPython-Greedy-DFS
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree_node::TreeNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        #[derive(PartialEq)]
        enum State {
            // if it is a leaf
            Leaf,
            // if it is a parent of a leaf, with a camera on this node
            ParentWithCamera,
            // if it is covered, without a camera on this node
            MonitoredWoCamera,
        }

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cnt: &Rc<RefCell<i32>>) -> State {
            if let Some(node) = root {
                let node = node.borrow();
                let left: State = dfs(&node.left, &cnt);
                let right: State = dfs(&node.right, &cnt);
                if left == State::Leaf || right == State::Leaf {
                    *cnt.borrow_mut() += 1;
                    return State::ParentWithCamera;
                }
                if left == State::ParentWithCamera || right == State::ParentWithCamera {
                    return State::MonitoredWoCamera;
                }
                return State::Leaf;
            }
            State::MonitoredWoCamera
        }
        let cnt: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
        let state = dfs(&root, &cnt);
        let count = *cnt.borrow();
        if state == State::Leaf {
            return 1 + count;
        }
        count
    }
}
