/// @author: Leon
/// https://leetcode.com/problems/insert-interval/
/// Time Complexity:    O(`len_is`)
/// Space Complexity:   O(1) / O(`len_is`)
/// Reference:
/// https://leetcode.com/problems/insert-interval/discuss/21602/Short-and-straight-forward-Java-solution/242084
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let len_is: usize = intervals.len();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_is);
        let mut prev = new_interval;
        for cur in intervals {
            if prev[1] < cur[0] {
                ans.push(prev.to_vec());
                prev = cur;
            } else if prev[0] > cur[1] {
                ans.push(cur);
            } else {
                prev[0] = std::cmp::min(prev[0], cur[0]);
                prev[1] = std::cmp::max(prev[1], cur[1]);
            }
        }
        ans.push(prev);
        return ans;
    }
}
