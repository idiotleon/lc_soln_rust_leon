/// https://leetcode.com/problems/house-robber/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/house-robber/discuss/156523/From-good-to-great.-How-to-approach-most-of-DP-problems.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        let mut cur_robbed = 0;
        let mut cur_not_robbed = 0;
        for num in nums {
            let prev_robbed = cur_robbed;
            let prev_not_robbed = cur_not_robbed;
            cur_robbed = cur_not_robbed + num;
            cur_not_robbed = std::cmp::max(prev_robbed, prev_not_robbed);
        }
        std::cmp::max(cur_robbed, cur_not_robbed)
    }
}
