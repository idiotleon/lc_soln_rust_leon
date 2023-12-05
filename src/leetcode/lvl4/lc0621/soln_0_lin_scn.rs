/// @author: Leon
/// https://leetcode.com/problems/task-scheduler/
/// Time Complexity:    O(`len_ts`)
/// Space Complexity:   O(1)
/// Reference:
/// http://zxi.mytechroad.com/blog/greedy/leetcode-621-task-scheduler/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let len_ts: usize = tasks.len();
        let freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 26];
            for task in tasks {
                freqs[task as usize - 'A' as usize] += 1;
            }
            freqs
        };
        let max_freq: i32 = *freqs.iter().max().unwrap_or(&0);
        let ans: i32 = {
            let mut ans: i32 = (max_freq - 1) * (n + 1);
            for freq in freqs {
                if freq == max_freq {
                    ans += 1;
                }
            }
            ans
        };
        return std::cmp::max(ans, len_ts as i32);
    }
}
