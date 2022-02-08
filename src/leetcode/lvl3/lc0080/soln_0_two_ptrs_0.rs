/// @author: Leon
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/discuss/27976/3-6-easy-lines-C%2B%2B-Java-Python-Ruby
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        for hi in 0..len_n {
            if lo < 2 || nums[hi - 2] < nums[hi] {
                nums[lo] = nums[hi];
                lo += 1;
            }
        }
        lo as i32
    }
}
