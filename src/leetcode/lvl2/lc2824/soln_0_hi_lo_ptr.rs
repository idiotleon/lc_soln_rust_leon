/// @author: Leon
/// https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/solutions/3933451/two-pointers-approach-easy-to-understand-in-9-languages/s
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - 1;
        let mut cnt: i32 = 0;
        while lo < hi {
            let sum = sorted[lo] + sorted[hi];
            if sum >= target {
                hi -= 1;
            } else {
                cnt += hi as i32 - lo as i32;
                lo += 1;
            }
        }
        return cnt;
    }
}
