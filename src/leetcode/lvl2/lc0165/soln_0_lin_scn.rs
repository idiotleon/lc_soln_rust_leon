/// @author: Leon
/// https://leetcode.com/problems/compare-version-numbers/
/// Time Complexity:    O(min(`len1`, `len2`))
/// Space Complexity:   O(max(`len1`, `len2`))
/// Reference:
/// https://leetcode.com/problems/compare-version-numbers/discuss/50774/Accepted-small-Java-solution./199430
/// https://leetcode.com/problems/compare-version-numbers/discuss/50774/Accepted-small-Java-solution.struct
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        const DOT: char = '.';
        const ZERO: char = '0';
        let len1: usize = version1.len();
        let chs1: Vec<char> = version1.chars().collect();
        let len2: usize = version2.len();
        let chs2: Vec<char> = version2.chars().collect();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        while idx1 < len1 || idx2 < len2 {
            let num1: i32 = {
                let mut num: i32 = 0;
                while idx1 < len1 && chs1[idx1] != DOT {
                    num = num * 10 + (chs1[idx1] as i32 - ZERO as i32);
                    idx1 += 1;
                }
                num
            };
            let num2: i32 = {
                let mut num: i32 = 0;
                while idx2 < len2 && chs2[idx2] != DOT {
                    num = num * 10 + (chs2[idx2] as i32 - ZERO as i32);
                    idx2 += 1;
                }
                num
            };
            if num1 != num2 {
                return if num1 > num2 { 1 } else { -1 };
            }
            idx1 += 1;
            idx2 += 1;
        }
        0
    }
}
