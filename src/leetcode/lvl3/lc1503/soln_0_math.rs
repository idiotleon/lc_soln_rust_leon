/// @author: Leon
/// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/
/// Time Complexity:    O(`_len_l` + `_len_r`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let _len_l: usize = left.len();
        let _len_r: usize = right.len();
        let mut ans: i32 = 0;
        for num in left {
            ans = std::cmp::max(ans, num);
        }
        for num in right {
            ans = std::cmp::max(ans, n - num);
        }
        return ans;
    }
}
