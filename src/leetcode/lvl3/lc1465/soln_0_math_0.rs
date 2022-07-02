use std::cmp::max;

/// @author: Leon
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
/// Time Complexity:    O(`_len_hcs` * lg(`_len_hcs`)) + O(`_len_vcs` * lg(`_len_vcs`))
/// Space Complexity:   O(`_len_hcs`) + O(`_len_vcs`)
/// Reference:
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/661995/Do-you-like-visual-explanation-You-got-it.-%3A-)-With-2-code-variations.
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/661644/C%2B%2BJava-Maximum-Gap-Between-Cuts
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let _len_hcs: usize = horizontal_cuts.len();
        let _len_vcs: usize = vertical_cuts.len();
        const MOD: i64 = 1e9 as i64 + 7;
        ((Self::get_max_gap(h, horizontal_cuts) as i64)
            * (Self::get_max_gap(w, vertical_cuts) as i64)
            % MOD) as i32
    }
    fn get_max_gap(boundary: i32, nums: Vec<i32>) -> i32 {
        let sorted = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let len_ns: usize = sorted.len();
        let max_gap = {
            let mut max_gap = max(boundary - sorted[len_ns - 1], sorted[0]);
            for idx in 1..len_ns {
                max_gap = max(max_gap, sorted[idx] - sorted[idx - 1]);
            }
            max_gap
        };
        max_gap
    }
}
