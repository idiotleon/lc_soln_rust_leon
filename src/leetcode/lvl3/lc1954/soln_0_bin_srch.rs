/// @author: Leon
/// https://leetcode.com/problems/minimum-garden-perimeter-to-collect-enough-apples/
/// Time Complexity:    O(lg(`DATA_RANGE`)), `DATA_RANGE` = lg(`needed_apples`) ^ (1 / 3)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-garden-perimeter-to-collect-enough-apples/discuss/1375478/JavaC%2B%2BPython-Binary-Search-and-O(1)-Soluitons
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        const RANGE: i64 = 1e5 as i64 + 7;
        let mut lo: i64 = 0;
        let mut hi: i64 = RANGE;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if mid * mid * mid * 4 + mid * mid * 6 + mid * 2 >= needed_apples {
                hi = mid
            } else {
                lo = mid + 1;
            }
        }
        lo * 8
    }
}
