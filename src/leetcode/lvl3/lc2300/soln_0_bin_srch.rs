/// @author: Leon
/// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
/// Time Complexity:    O(`_len_ss` * lg(`len_ps`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let _len_ss: usize = spells.len();
        let len_ps: usize = potions.len();
        let potions = {
            let mut sorted = potions;
            sorted.sort();
            sorted
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_ps);
        for sp in spells {
            let idx = Self::search(&potions, sp as i64, success);
            if idx == -1 {
                ans.push(0);
            } else {
                ans.push((len_ps - idx as usize) as i32);
            }
        }
        return ans;
    }
    fn search(potions: &Vec<i32>, spell: i64, success: i64) -> isize {
        let len_ps: usize = potions.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_ps as isize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let value = potions[mid as usize] as i64 * spell;
            if value >= success {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo == len_ps as isize {
            return -1;
        }
        return if potions[lo as usize] as i64 * spell >= success {
            lo
        } else {
            -1
        };
    }
}
