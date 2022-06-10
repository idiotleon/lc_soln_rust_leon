use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/replace-elements-in-an-array/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let mut val_to_idx: HashMap<i32, (i32, usize)> = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            val_to_idx.insert(num, (num, idx));
        }
        for op in operations {
            let val = op[0];
            let nval = op[1];
            if let Some(&(_, idx)) = val_to_idx.get(&val) {
                val_to_idx.insert(nval, (nval, idx));
                val_to_idx.remove(&val);
            }
        }
        let mut ans: Vec<(i32, usize)> = val_to_idx.into_values().collect();
        ans.sort_by_key(|e| e.1);
        ans.into_iter().map(|e| e.0).collect()
    }
}
