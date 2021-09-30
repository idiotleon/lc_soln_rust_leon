/// https://leetcode.com/problems/jump-game/
///
/// Time Complexity:     O(`size`)
/// Space Complexity:    O(1)
///
/// Reference:
/// https://leetcode.com/problems/jump-game/discuss/20917/Linear-and-simple-solution-in-C++/20979
use std::cmp;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut furthest: usize = 0;
        let mut i: usize = 0;

        while i <= furthest {
            furthest = cmp::max(furthest, i + nums[i] as usize);

            if furthest >= size - 1 {
                return true;
            }

            i += 1;
        }

        false
    }
}
