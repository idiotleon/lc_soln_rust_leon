/// @author: Leon
/// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/
/// Time Complexity:    O(`_len_wts` * lg(sum(`weights`) - max(`weights`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let _len_wts: usize = weights.len();
        let mut lo: i32 = *weights.iter().max().unwrap();
        let mut hi: i32 = weights.iter().sum::<i32>();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut need = 1;
            let mut cur = 0;
            for &weight in &weights {
                if cur + weight > mid {
                    need += 1;
                    cur = 0;
                }
                cur += weight;
            }
            if need > days {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
}
