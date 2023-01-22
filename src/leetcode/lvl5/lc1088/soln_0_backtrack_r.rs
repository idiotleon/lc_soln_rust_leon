use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/confusing-number-ii/
/// Time Complexity:    O(len_of_n * 6) ~ O(len_of_n)
/// Space Complexity:   O(6) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/confusing-number-ii/discuss/446589/Easy-to-understand-Java-backtracking-solution-covers-edge-case
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn confusing_number_ii(n: i32) -> i32 {
        // candidates to their rotated equivalents
        let cand_to_rot: HashMap<i64, i64> = {
            let mut map: HashMap<i64, i64> = HashMap::with_capacity(6);
            map.insert(0, 0);
            map.insert(1, 1);
            map.insert(6, 9);
            map.insert(8, 8);
            map.insert(9, 6);
            map
        };
        let mut cnt: i32 = 0;
        Self::backtrack(0, n as i64, &cand_to_rot, &mut cnt);
        return cnt;
    }
    fn backtrack(num_cur: i64, n: i64, cand_to_rot: &HashMap<i64, i64>, cnt: &mut i32) {
        if Self::is_confusing(num_cur, cand_to_rot) {
            *cnt += 1;
        }
        for (&candidate, &_rotated) in cand_to_rot.iter() {
            let num_nxt = num_cur * 10 + candidate;
            if num_nxt <= n && num_nxt != 0 {
                Self::backtrack(num_nxt, n, cand_to_rot, cnt);
            }
        }
    }
    fn is_confusing(n: i64, cand_to_rot: &HashMap<i64, i64>) -> bool {
        let mut num = n;
        let mut rotated = 0;
        while num > 0 {
            rotated = rotated * 10
                + if let Some(&cand) = cand_to_rot.get(&(num % 10)) {
                    cand
                } else {
                    0
                };
            num /= 10;
        }
        // to make sure it is NOT the original `num`
        return rotated as i64 != n;
    }
}
