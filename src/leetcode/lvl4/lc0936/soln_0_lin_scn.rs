/// @author: Leon
/// https://leetcode.com/problems/stamping-the-sequence/
/// Time Complexity:     O((`lenS` ^ 2) * (`lenT` - `lenS`))
/// Space Complexity:    O(`lenT`)
/// Reference:
/// https://leetcode.com/problems/stamping-the-sequence/discuss/201546/12ms-java-solution-beats-100/1553113
struct Solution;

#[allow(dead_code)]
impl Solution {
    const PLACEHOLDER: char = '#';
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let len_s: usize = stamp.len();
        let len_t: usize = target.len();
        let chs_s: Vec<char> = stamp.chars().collect();
        let mut chs_t: Vec<char> = target.chars().collect();
        let mut ans: Vec<i32> = Vec::with_capacity(len_s);
        let mut seen: Vec<bool> = vec![false; len_t];
        let mut idx: usize = 0;
        while idx < len_t {
            let mut found = false;
            for i in 0..=len_t - len_s {
                if seen[i] || !Self::can_replace(i, &chs_t, &chs_s) {
                    continue;
                }
                idx += Self::replace(i, &mut chs_t, len_s);
                found = true;
                seen[i] = true;
                ans.push(i as i32);
                if idx == len_t {
                    break;
                }
            }
            if !found {
                return vec![];
            }
        }
        ans.reverse();
        return ans;
    }
    fn replace(idx_start: usize, chs_t: &mut Vec<char>, len_s: usize) -> usize {
        let mut cnt: usize = 0;
        for idx in 0..len_s {
            if chs_t[idx_start + idx] == Self::PLACEHOLDER {
                continue;
            }
            chs_t[idx_start + idx] = Self::PLACEHOLDER;
            cnt += 1;
        }
        return cnt;
    }
    fn can_replace(idx_start: usize, chs_t: &Vec<char>, chs_s: &Vec<char>) -> bool {
        let len_s: usize = chs_s.len();
        for idx in 0..len_s {
            if chs_t[idx_start + idx] != Self::PLACEHOLDER && chs_t[idx_start + idx] != chs_s[idx] {
                return false;
            }
        }
        return true;
    }
}
