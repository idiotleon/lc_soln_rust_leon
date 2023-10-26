/// @author: Leon
/// https://leetcode.com/problems/first-bad-version/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo: i32 = 1;
        // cannot use `n + 1` here,
        // to prevent the i32 type overflow
        let mut hi: i32 = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.is_bad_version(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo;
    }
    // fake API
    #[allow(unused_variables)]
    fn is_bad_version(&self, version: i32) -> bool {
        return true;
    }
}
