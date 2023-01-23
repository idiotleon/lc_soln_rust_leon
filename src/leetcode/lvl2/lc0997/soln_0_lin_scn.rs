/// @author: Leon
/// https://leetcode.com/problems/find-the-town-judge/
/// Time Complexity:    O(`_len_ts`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }
        let _len_ts: usize = trust.len();
        let (indegrees, outdegrees): (Vec<i32>, Vec<i32>) = {
            let mut indegrees: Vec<i32> = vec![0; n as usize + 1];
            let mut outdegrees: Vec<i32> = vec![0; n as usize + 1];
            for t in trust {
                let a = t[0] as usize;
                let b = t[1] as usize;
                outdegrees[a] += 1;
                indegrees[b] += 1;
            }
            (indegrees, outdegrees)
        };
        for (idx, degree) in indegrees.into_iter().enumerate() {
            if degree == n - 1 {
                if outdegrees[idx] == 0 {
                    return idx as i32;
                }
            }
        }
        return -1;
    }
}
