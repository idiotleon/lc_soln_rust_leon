/// @author: Leon
/// https://leetcode.com/problems/verifying-an-alien-dictionary/
/// Time Complexity:    O(n_words * avg_word_len)
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let len_ws: usize = words.len();
        let order_vec: Vec<usize> = {
            let mut order_vec: Vec<usize> = vec![0; 26];
            for (idx, ch) in order.chars().into_iter().enumerate() {
                order_vec[ch as usize - 'a' as usize] = idx;
            }
            order_vec
        };
        let chses: Vec<Vec<char>> = words
            .into_iter()
            .map(|word| word.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        for idx in 1..len_ws {
            let chs_prev = &chses[idx - 1];
            let chs_cur = &chses[idx];
            if Self::is_bigger(chs_prev, chs_cur, &order_vec) {
                return false;
            }
        }
        return true;
    }
    fn is_bigger(chs_prev: &Vec<char>, chs_cur: &Vec<char>, order: &Vec<usize>) -> bool {
        let len_prev: usize = chs_prev.len();
        let len_cur: usize = chs_cur.len();
        let len_min: usize = std::cmp::min(len_prev, len_cur);
        for idx in 0..len_min {
            let ch_prev = chs_prev[idx];
            let ch_cur = chs_cur[idx];
            if ch_prev != ch_cur {
                return order[ch_prev as usize - 'a' as usize]
                    > order[ch_cur as usize - 'a' as usize];
            }
        }
        return len_prev > len_cur;
    }
}
