/// @author: Leon
/// https://leetcode.com/problems/two-sum-less-than-k/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
/// Reference:
/// https://leetcode.com/problems/two-sum-less-than-k/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        const RANGE: usize = 1000;
        let _len_ns: usize = nums.len();
        let mut ans: i32 = -1;
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; RANGE + 1];
            for num in nums {
                freqs[num as usize] += 1;
            }
            freqs
        };
        let mut lo: usize = 1;
        let mut hi: usize = RANGE;
        while lo <= hi {
            if lo + hi >= k as usize || freqs[hi] == 0 {
                hi -= 1;
            } else {
                // > 0, when `lo` < `hi`
                // > 1, when `lo` == `hi`
                if freqs[lo] > if lo < hi { 0 } else { 1 } {
                    ans = std::cmp::max(ans, (lo + hi) as i32);
                }
                lo += 1;
            }
        }
        return ans;
    }
}
