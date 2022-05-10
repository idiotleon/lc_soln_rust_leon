use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let mut num_to_idx: HashMap<i32, usize> = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate(){
            let expected = target - num;
            if num_to_idx.contains_key(&expected){
                return vec![*num_to_idx.get(&expected).unwrap() as i32 + 1, idx as i32 + 1];
            }
            num_to_idx.insert(num, idx);
        }
        unreachable!()
    }
}