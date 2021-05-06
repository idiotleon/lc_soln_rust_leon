// https://leetcode.com/problems/jump-game/
//
// Time Complexity:     O(`size`)
// Space Complexity:    O(1)
//
// based on the solution template of Jump Game ii
//
// Reference:
//  https://leetcode.com/problems/jump-game-ii/discuss/18014/Concise-O(n)-one-loop-JAVA-solution-based-on-Greedy
use std::cmp;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let size: usize = nums.len();
        let mut cur_end: usize = 0;
        let mut cur_furthest: usize = 0;
        
        for i in 0..size - 1{
            cur_furthest = cmp::max(cur_furthest, i + nums[i] as usize);
            
            if i == cur_end{
                cur_end = cur_furthest;
                
                if cur_furthest >= size - 1{
                    return true;
                }
            }
        }
        
        size == 1
    }
}