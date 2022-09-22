/// @author: Leon
/// https://leetcode.com/problems/remove-interval/
/// Time Complexity:    O(`len_is`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-interval/discuss/440799/JavaPython-3-12-and-11-liners-w-brief-comments-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
        let len_is: usize = intervals.len();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_is);
        let start_r: i32 = to_be_removed[0];
        let end_r: i32 = to_be_removed[1];
        for interval in intervals {
            let start = interval[0];
            let end = interval[1];
            if end <= start_r || start >= end_r {
                ans.push(vec![start, end]);
            } else {
                if start < start_r {
                    ans.push(vec![start, start_r]);
                }
                if end_r < end {
                    ans.push(vec![end_r, end]);
                }
            }
        }
        return ans;
    }
}
