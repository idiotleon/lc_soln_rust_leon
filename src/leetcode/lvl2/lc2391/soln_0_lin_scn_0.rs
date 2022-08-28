/// @author: Leon
/// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/
/// Time Complexity:    O(`_len_gs` * avg_len_word)
/// Space Complexity:   O(avg_len_word)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn garbage_collection(garbages: Vec<String>, travel: Vec<i32>) -> i32 {
        const M: char = 'M';
        const G: char = 'G';
        const P: char = 'P';
        let types: Vec<char> = vec![M, G, P];
        let _len_gs: usize = garbages.len();
        let mut ans: i32 = 0;
        for t in types{
            let idx_last: usize = Self::get_last_idx(&garbages, t);
            ans += Self::get_time(&garbages, &travel, idx_last, t);
        }
        return ans;
    }
    fn get_time(garbages: &Vec<String>, travel: &Vec<i32>, last_idx: usize, target: char) -> i32 {
        let len_gs: usize = garbages.len();
        if last_idx == len_gs {
            return 0;
        }
        let mut time: i32 = 0;
        for idx in 0..=last_idx {
            let g = &garbages[idx];
            time += g.chars().filter(|&ch| ch == target).count() as i32;
            if idx > 0 {
                time += travel[idx - 1];
            }
        }
        return time;
    }
    fn get_last_idx(garbages: &Vec<String>, target: char) -> usize {
        let len_gs: usize = garbages.len();
        let mut last_idx: usize = len_gs;
        for (idx, g) in garbages.iter().enumerate().rev() {
            if let Some(_) = g.find(target) {
                last_idx = idx;
                break;
            }
        }
        return last_idx;
    }
}
