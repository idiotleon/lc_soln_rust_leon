/// @author: Leon
/// https://leetcode.com/problems/partition-array-according-to-given-pivot/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/partition-array-according-to-given-pivot/discuss/1747184/JavaPython-3-2-methods-w-brief-explanation-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut ans: Vec<i32> = vec![0; len_n];
        let mut cnt_pivot: u32 = 0;
        let mut idx: usize = 0;
        for &num in &nums {
            if num < pivot {
                ans[idx] = num;
                idx += 1;
            } else if num == pivot {
                cnt_pivot += 1;
            }
        }
        while cnt_pivot > 0 {
            ans[idx] = pivot;
            idx += 1;
            cnt_pivot -= 1;
        }
        for num in nums {
            if num > pivot {
                ans[idx] = num;
                idx += 1;
            }
        }
        ans
    }
}
