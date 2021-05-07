/// https://leetcode.com/problems/two-sum/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// References:
///     https://leetcode.com/problems/two-sum/discuss/715951/Rust%3A-HashMap-solution
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_idx: HashMap<i32, i32> = HashMap::new();
        
        for (idx, num) in nums.iter().enumerate(){
            match num_to_idx.get(&(target - *num)){
                Some(&idx2) => return vec![idx as i32, idx2],
                None => num_to_idx.insert(*num, idx as i32),
            };
        }
        
        unreachable!()
    }
}