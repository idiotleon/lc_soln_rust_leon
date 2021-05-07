// https://leetcode.com/problems/jump-game/
//
// Time Complexity:     O(`len`)
// Space Complexity:    O(1)
//
// Reference:
//  https://leetcode.com/problems/jump-game/discuss/20917/Linear-and-simple-solution-in-C++/187269
use std::cmp;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // not used
        // let len = nums.len();
        let mut furthest: usize = 0;
        
        for (i, &num) in nums.iter().enumerate(){
            if furthest < i{
                return false;
            }
            
            furthest = cmp::max(furthest, i + num as usize);
        }
        
        true
    }
}