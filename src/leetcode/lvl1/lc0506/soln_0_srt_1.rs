/// @author: Leon
/// https://leetcode.com/problems/relative-ranks/
/// Time Complexity:    O(`len_ss` * lg(`len_ss`))
/// Space Complexity:   O(1) / O(`len_ss`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_relative_ranks(scores: Vec<i32>) -> Vec<String> {
        const MEDAL_GOLD: &str = "Gold Medal";
        const MEDAL_SILVER: &str = "Silver Medal";
        const MEDAL_BRONZE: &str = "Bronze Medal";
        let len_ss: usize = scores.len();
        let sorted: Vec<(i32, usize)> = {
            let mut sorted: Vec<(i32, usize)> = scores
                .into_iter()
                .enumerate()
                .map(|(idx, num)| (num, idx))
                .collect();
            sorted.sort_by_key(|&(num, _idx)| num);
            sorted
        };
        let mut ans: Vec<String> = vec!["".to_owned(); len_ss];
        let mut rank: i32 = 4;
        for (idx_sorted, (_num, idx)) in sorted.into_iter().enumerate().rev() {
            if idx_sorted == len_ss - 1 {
                ans[idx] = MEDAL_GOLD.to_owned();
            } else if idx_sorted == len_ss - 2 {
                ans[idx] = MEDAL_SILVER.to_owned();
            } else if idx_sorted == len_ss - 3 {
                ans[idx] = MEDAL_BRONZE.to_owned();
            } else {
                ans[idx] = rank.to_string();
                rank += 1;
            }
        }
        return ans;
    }
}
