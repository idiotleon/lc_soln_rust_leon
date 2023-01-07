use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/relative-ranks/
/// Time Complexity:    O(`len_ss` * lg(`len_ss`))
/// Space Complexity:   O(`len_ss`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_relative_ranks(scores: Vec<i32>) -> Vec<String> {
        const MEDAL_GOLD: &str = "Gold Medal";
        const MEDAL_SILVER: &str = "Silver Medal";
        const MEDAL_BRONZE: &str = "Bronze Medal";
        let len_ss: usize = scores.len();
        let score_to_rank: HashMap<i32, usize> = {
            let mut map: HashMap<i32, usize> = HashMap::with_capacity(len_ss);
            let mut sorted = scores.to_vec();
            sorted.sort();
            for (idx, num) in sorted.into_iter().enumerate() {
                map.insert(num, len_ss - idx);
            }
            map
        };
        let mut ans: Vec<String> = Vec::with_capacity(len_ss);
        for score in scores {
            if let Some(rank) = score_to_rank.get(&score) {
                let str_rank = match rank {
                    1 => MEDAL_GOLD.to_owned(),
                    2 => MEDAL_SILVER.to_owned(),
                    3 => MEDAL_BRONZE.to_owned(),
                    _ => rank.to_string(),
                };
                ans.push(str_rank);
            }
        }
        return ans;
    }
}
