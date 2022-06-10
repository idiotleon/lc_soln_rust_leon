/// @author: Leon
/// https://leetcode.com/problems/min-max-game/
/// Time Complexity:    O(lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let mut nums = nums;
        while nums.len() > 1 {
            let len_n: usize = nums.len();
            let mut res: Vec<i32> = vec![];
            for idx in 0..len_n / 2 {
                if idx % 2 == 0 {
                    res.push(std::cmp::min(nums[2 * idx], nums[2 * idx + 1]));
                } else {
                    res.push(std::cmp::max(nums[2 * idx], nums[2 * idx + 1]));
                }
            }
            nums = res;
        }
        nums[0]
    }
}
