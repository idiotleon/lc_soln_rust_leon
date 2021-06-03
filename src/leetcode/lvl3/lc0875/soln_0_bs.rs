/// https://leetcode.com/problems/koko-eating-bananas/
///
/// Time Complexity:    O(`len_s` * `DATA_RANGE`)
/// Space Complexity:   O(1)
///
/// Reference:
/// https://leetcode.com/problems/koko-eating-bananas/discuss/152324/JavaC%2B%2BPython-Binary-Search
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // not used
        // let len_ps = piles.len();
        const DATA_RANGE: usize = 1e9 as usize + 7;
        let mut lo: usize = 1;
        let mut hi: usize = DATA_RANGE;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut total: i32 = 0;

            for pile in &piles {
                total += (pile + mid as i32 - 1) / mid as i32;
            }

            if total > h {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}
