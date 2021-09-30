/// @author: Leon
/// https://leetcode.com/problems/verifying-an-alien-dictionary/
///
/// Time Complexity:    O(n_words * avg_word_len)
/// Space Complexity:   O(26) ~ O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        fn is_bigger(chs_prev: &Vec<char>, chs_cur: &Vec<char>, dict: &Vec<usize>) -> bool {
            let len_prev = chs_prev.len();
            let len_cur = chs_cur.len();
            let len = std::cmp::min(len_prev, len_cur);

            for idx in 0..len {
                let ch_prev: char = chs_prev[idx];
                let ch_cur: char = chs_cur[idx];

                if ch_prev != ch_cur {
                    return dict[ch_prev as usize - 'a' as usize]
                        > dict[ch_cur as usize - 'a' as usize];
                }
            }

            len_prev > len_cur
        }

        let chses: Vec<Vec<char>> = words.into_iter().map(|w| w.chars().collect()).collect();
        let dict: Vec<usize> = {
            let mut dict: Vec<usize> = vec![0; 26];
            for (idx, ch) in order.chars().into_iter().enumerate() {
                dict[ch as usize - 'a' as usize] = idx;
            }

            dict
        };

        let len_chses = chses.len();
        for idx in 1..len_chses {
            if is_bigger(&chses[idx - 1], &chses[idx], &dict) {
                return false;
            }
        }

        true
    }
}
