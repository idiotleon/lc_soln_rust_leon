/// https://leetcode.com/problems/two-sum/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// References:
///     https://leetcode.com/problems/two-sum/discuss/207679/Rust
/// 
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for lo in 0..nums.len(){
            for hi in lo + 1..nums.len(){
                if nums[lo] + nums[hi] == target {
                    return vec![lo as i32, hi as i32]
                }
            }
        }
        
        unreachable!()
    }
}