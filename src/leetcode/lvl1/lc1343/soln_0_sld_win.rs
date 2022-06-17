/// @author: Leon
/// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(nums: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let _len_n: usize = nums.len();
        let len_k: usize = k as usize;
        let mut cnt: i32 = 0;
        let mut sum: i32 = 0;
        for (idx, &num) in nums.iter().enumerate(){
            sum += num;
            if idx + 1 >= len_k{
                if sum / k >= threshold{
                    cnt += 1;
                }
                sum -= nums[idx + 1 - len_k];
            }
        }
        cnt
    }
}