/// @author: Leon
/// https://leetcode.com/problems/find-target-indices-after-sorting-array/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1) / O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/find-target-indices-after-sorting-array/discuss/1599800/C%2B%2B-O(N)-Time-Counting-Sort
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let mut cnt = 0;
        let mut rank = 0;
        for num in nums {
            if num == target {
                cnt += 1;
            } else if num < target {
                rank += 1;
            }
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::new();
            while cnt > 0 {
                res.push(rank);
                cnt -= 1;
                rank += 1;
            }
            res
        };
        ans
    }
}
