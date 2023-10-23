/// @author: Leon
/// https://leetcode.com/problems/merge-intervals/
/// Time Complexity:    O(`len_is` * lg(`len_is`))
/// Space Complexity:   O(`len_is`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_is: usize = intervals.len();
        let sorted: Vec<Vec<i32>> = {
            let mut sorted = intervals;
            sorted.sort_by_key(|v| v[0]);
            sorted
        };
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_is);
        for interval in sorted {
            if ans.is_empty() || ans.last().unwrap()[1] < interval[0] {
                ans.push(interval);
            } else if ans.last().unwrap()[1] >= interval[0] {
                if let Some(last) = ans.last_mut() {
                    last[1] = std::cmp::max(last[1], interval[1]);
                }
            }
        }
        return ans;
    }
}
