/// @author: Leon
/// https://leetcode.com/problems/cutting-ribbons/
/// Time Complexity:    O(`_len_r` * lg(`RANGE`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/cutting-ribbons/discuss/1263437/Java-Simple-Binary-Search-%2B-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_length(ribbons: Vec<i32>, k: i32) -> i32 {
        let _len_r: usize = ribbons.len();
        const RANGE: i32 = 1e5 as i32 + 7;
        let mut lo: usize = 1;
        let mut hi: usize = RANGE as usize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if !Self::is_cut_eligible(mid as i32, k, &ribbons) {
                hi = mid;
            } else {
                lo = 1 + mid;
            }
        }
        lo as i32 - 1
    }
    fn is_cut_eligible(len: i32, k: i32, ribbons: &Vec<i32>) -> bool {
        let mut cnt = 0;
        for ribbon in ribbons {
            cnt += ribbon / len;
        }
        cnt >= k
    }
}
