/// @author: Leon
/// https://leetcode.com/problems/bag-of-tokens/
/// Time Complexity:    O(`len_ts` * lg(`len_ts`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/bag-of-tokens/discuss/197696/C%2B%2BJavaPython-Greedy-%2B-Two-Pointers
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let len_ts: usize = tokens.len();
        let tokens: Vec<i32> = {
            let mut tokens = tokens;
            tokens.sort();
            tokens
        };
        let mut lo: usize = 0;
        let mut hi: usize = len_ts - 1;
        let mut most: i32 = 0;
        let mut pts: i32 = 0;
        while lo <= hi {
            if power >= tokens[lo] {
                power -= tokens[lo];
                lo += 1;
                pts += 1;
                most = std::cmp::max(most, pts);
            } else if pts > 0 {
                pts -= 1;
                power += tokens[hi];
                hi -= 1;
            } else {
                break;
            }
        }
        return most;
    }
}
