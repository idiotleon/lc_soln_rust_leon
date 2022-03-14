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
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        let mut i: usize = 0;
        let mut j: usize = len_n - 1;
        while i < len_n {
            if nums[i] < pivot {
                ans[lo] = nums[i];
                lo += 1;
            }
            if nums[j] > pivot {
                ans[hi] = nums[j];
                hi -= 1;
            }
            i += 1;
            j -= 1;
        }
        while lo <= hi {
            ans[lo] = pivot;
            lo += 1;
        }
        ans
    }
}
