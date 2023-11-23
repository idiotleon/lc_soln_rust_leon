use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/arithmetic-subarrays/
/// Time Complexity:    O(`_len_ns` * `len_ms`)
/// Space Complexity:   O(`_len_ns`)
/// Reference:
/// https://leetcode.com/problems/arithmetic-subarrays/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: i32 = 1e5 as i32 + 7;
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let _len_ns: usize = nums.len();
        let len_ms: usize = l.len();
        let mut ans: Vec<bool> = Vec::with_capacity(len_ms);
        for idx in 0..len_ms {
            let lo: usize = l[idx] as usize;
            let hi: usize = r[idx] as usize;
            ans.push(Self::is_valid(lo, hi, &nums));
        }
        return ans;
    }
    fn is_valid(lo: usize, hi: usize, nums: &Vec<i32>) -> bool {
        let len: usize = hi - lo + 1;
        let (max, min, seen): (i32, i32, HashSet<i32>) = {
            let mut idx: usize = lo;
            let mut max: i32 = i32::MIN;
            let mut min: i32 = i32::MAX;
            let mut seen: HashSet<i32> = HashSet::with_capacity(len);
            while idx <= hi {
                max = std::cmp::max(max, nums[idx]);
                min = std::cmp::min(min, nums[idx]);
                seen.insert(nums[idx]);
                idx += 1;
            }
            (max, min, seen)
        };
        if (max - min) % (len as i32 - 1) != 0 {
            return false;
        }
        let diff_exp: i32 = (max - min) / (len as i32 - 1);
        let mut cur = min + diff_exp;
        while cur < max {
            if !seen.contains(&cur) {
                return false;
            }
            cur += diff_exp;
        }
        return true;
    }
}
