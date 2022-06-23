/// @author: Leon
/// https://leetcode.com/problems/jump-game-ii/
/// Time Complexity:     O(`len_n`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/jump-game-ii/discuss/18014/Concise-O(n)-one-loop-JAVA-solution-based-on-Greedy
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut jumps: i32 = 0;
        let mut cur_end: usize = 0;
        let mut cur_furthest: usize = 0;
        for i in 0..len_n - 1 {
            cur_furthest = std::cmp::max(cur_furthest, i + nums[i] as usize);
            if i == cur_end {
                jumps += 1;
                cur_end = cur_furthest;
                if cur_end >= len_n - 1 {
                    break;
                }
            }
        }
        jumps
    }
}
