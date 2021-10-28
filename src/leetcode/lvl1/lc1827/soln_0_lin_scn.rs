/// @author: Leon
/// https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/
///
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        let mut cur_max = 0;
        let mut cnt = 0;
        for num in nums {
            let expect = cur_max + 1;
            if expect > num {
                cnt += expect - num;
            }
            cur_max = std::cmp::max(expect, num);
        }
        cnt
    }
}
