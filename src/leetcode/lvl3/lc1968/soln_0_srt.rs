/// @author: Leon
/// https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/
/// Time Complexity:    O(`len_N` * lg(`len_n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/discuss/1403927/JavaC%2B%2BPython-Easy-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut nums: Vec<i32> = nums;
        nums.sort();
        for idx in (1..len_n).step_by(2){
            nums.swap(idx - 1, idx);
        }
        nums
    }
}