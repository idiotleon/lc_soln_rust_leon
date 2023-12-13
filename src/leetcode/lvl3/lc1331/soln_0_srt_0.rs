/// @author: Leon
/// https://leetcode.com/problems/rank-transform-of-an-array/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_rank_transform(nums: Vec<i32>) -> Vec<i32> {
        const RANGE: i32 = 1e9 as i32;
        const IMPS: i32 = RANGE + 7;
        let len_ns: usize = nums.len();
        let sorted: Vec<(i32, usize)> = {
            let mut res: Vec<(i32, usize)> = nums
                .into_iter()
                .enumerate()
                .map(|(idx, num)| (num, idx))
                .collect();
            res.sort_by_key(|r| r.0);
            res
        };
        let mut ans: Vec<i32> = vec![0; len_ns];
        let mut prev_val: i32 = -IMPS;
        let mut prev_rank: i32 = -(len_ns as i32);
        let mut rank: i32 = 1;
        for (num, idx) in sorted.into_iter() {
            if prev_val == num {
                ans[idx] = prev_rank;
            } else {
                ans[idx] = rank;
                prev_val = num;
                prev_rank = rank;
                rank += 1;
            }
        }
        return ans;
    }
}
