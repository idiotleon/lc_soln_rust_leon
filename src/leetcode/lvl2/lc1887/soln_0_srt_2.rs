/// @author: Leon
/// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        if sorted[0] == sorted[len_ns - 1] {
            return 0;
        }
        let mut ans: i32 = 0;
        for idx in (1..len_ns).rev() {
            if sorted[idx - 1] == sorted[0] {
                ans += (len_ns - idx) as i32;
                break;
            }
            if sorted[idx - 1] != sorted[idx] {
                ans += (len_ns - idx) as i32;
            }
        }
        return ans;
    }
}
