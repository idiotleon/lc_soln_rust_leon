/// @author: Leon
/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/discuss/212631/Rust-12ms-Solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let idxed = {
            let mut tmp = nums;
            for i in 0..len_n {
                let idx = (tmp[i].abs() - 1) as usize;
                tmp[idx] = -tmp[idx].abs();
            }
            tmp
        };
        let ans = {
            let mut ans = vec![];
            for (idx, n) in idxed.into_iter().enumerate() {
                if n > 0 {
                    ans.push(idx as i32 + 1);
                }
            }
            ans
        };
        ans
    }
}
