/// @author: Leon
/// https://leetcode.com/problems/two-furthest-houses-with-different-colors/
/// Time Complexity:    O(`len_cs`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/two-furthest-houses-with-different-colors/discuss/1589141/JavaC%2B%2BPython-O(n)-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let len_cs: usize = colors.len();
        let mut lo: usize = 0;
        // according to the description,
        // it is guaranteed that there are more then one colors,
        // so that `hi` does not have be of type `isize`
        let mut hi: usize = len_cs - 1;
        while colors[0] == colors[hi] {
            hi -= 1;
        }
        while colors[lo] == colors[len_cs - 1] {
            lo += 1;
        }
        return std::cmp::max(len_cs - 1 - lo, hi) as i32;
    }
}
