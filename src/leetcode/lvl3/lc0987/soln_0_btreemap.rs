use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::Reverse;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/discuss/231148/Java-TreeMap-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map: BTreeMap<i16, BTreeMap<i16, BinaryHeap<Reverse<i32>>>> = BTreeMap::new();
        Self::dfs((0, 0), root, &mut map);
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for (_, m) in map{
            let mut col: Vec<i32> = Vec::new();
            for (_, mut heap) in m{
                while let Some(Reverse(num)) = heap.pop(){
                    col.push(num);
                }
            }
            ans.push(col);
        }
        return ans;
    }
    fn dfs(coord: (i16, i16), node: Option<Rc<RefCell<TreeNode>>>, map: &mut BTreeMap<i16, BTreeMap<i16, BinaryHeap<Reverse<i32>>>>){
        if let Some(n) = node{
            let (r, c) = coord;
            let value = n.borrow().val;
            map.entry(r).or_default().entry(c).or_default().push(Reverse(value));
            Self::dfs((r - 1, c + 1), n.borrow().left.clone(), map);
            Self::dfs((r + 1, c + 1), n.borrow().right.clone(), map);
        }
    }
}