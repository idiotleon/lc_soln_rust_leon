use std::cmp::max;

/// @author: Leon
/// https://leetcode.com/problems/maximum-gap/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/maximum-gap/discuss/756567/Rust-translated
/// https://leetcode.com/problems/maximum-gap/discuss/50694/12ms-C%2B%2B-Suggested-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        if len_n < 2 {
            return 0;
        }
        let max_num: usize = *nums.iter().max().unwrap() as usize;
        let min_num: usize = *nums.iter().min().unwrap() as usize;
        let bkt_sze: usize = max(1, (max_num - min_num) / len_n);
        let bkt_cnt: usize = 1 + (max_num - min_num) / bkt_sze;
        let bkts: Vec<(Option<i32>, Option<i32>)> = {
            let mut bkts: Vec<(Option<i32>, Option<i32>)> = vec![(None, None); bkt_cnt];
            for num in nums.into_iter() {
                let idx_bkt = (num as usize - min_num) / bkt_sze;
                match bkts[idx_bkt] {
                    (Some(a), Some(b)) => {
                        if num < a {
                            bkts[idx_bkt] = (Some(num), Some(b))
                        } else if num > b {
                            bkts[idx_bkt] = (Some(a), Some(num))
                        }
                    }
                    (None, None) => bkts[idx_bkt] = (Some(num), Some(num)),
                    _ => (),
                }
            }
            bkts
        };
        let mut max_gap: i32 = 0;
        let mut prev_max: i32 = min_num as i32;
        for bkt in bkts.into_iter() {
            if let (Some(a), Some(b)) = bkt {
                max_gap = max(max_gap, a - prev_max);
                prev_max = b;
            }
        }
        max_gap
    }
}
