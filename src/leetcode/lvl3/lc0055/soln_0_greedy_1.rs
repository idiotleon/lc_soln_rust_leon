// https://leetcode.com/problems/jump-game/
//
// Time Complexity:     O(`size`)
// Space Complexity:    O(1)
//
// Reference:
//  https://leetcode.com/problems/jump-game/discuss/20917/Linear-and-simple-solution-in-C++/187269
use std::cmp;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut furthest = 0;
        
        for (i, &num) in nums.iter().enumerate(){
            if furthest < i{
                return false;
            }
            
            furthest = cmp::max(furthest, i + num as usize);
        }
        
        true
    }
}