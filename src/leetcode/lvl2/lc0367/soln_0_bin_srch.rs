/// @author: Leon
/// https://leetcode.com/problems/valid-perfect-square/
/// Time Complexity:    O(lg(`num`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/valid-perfect-square/solutions/83874/a-square-number-is-1-3-5-7-java-code/?orderBy=hot
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num: i64 = num as i64;
        let mut lo: i64 = 1;
        let mut hi: i64 = num;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let square = mid * mid;
            if square == num {
                return true;
            }
            if square < num {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        return false;
    }
}
