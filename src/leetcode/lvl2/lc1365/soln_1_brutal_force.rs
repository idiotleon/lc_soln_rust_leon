/// @author: Leon
/// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut ans: Vec<i32> = vec![0; len_n];
        for idx1 in 0..len_n {
            let mut cnt: i32 = 0;
            for idx2 in 0..len_n {
                if idx1 == idx2 {
                    continue;
                }
                if nums[idx2] < nums[idx1] {
                    cnt += 1;
                }
            }
            ans[idx1] = cnt;
        }
        ans
    }
}
