/// https://leetcode.com/problems/set-mismatch/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/set-mismatch/discuss/105507/Java-O(n)-Time-O(1)-Space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut num_ans: i32 = 0;
        let mut nums = nums;
        for idx in 0..len_n {
            let num = nums[idx];
            if nums[num.abs() as usize - 1] < 0 {
                num_ans = num.abs();
            } else {
                nums[num.abs() as usize - 1] *= -1;
            }
        }
        for (idx, num) in nums.into_iter().enumerate() {
            if num > 0 {
                return vec![num_ans, idx as i32 + 1];
            }
        }
        unreachable!();
    }
}
