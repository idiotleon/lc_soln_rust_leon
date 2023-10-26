use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/alien-dictionary/
/// Time Complexity:    O(`len_ws` * avg_len_word)
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let len_ws: usize = words.len();
        let chses: Vec<Vec<char>> = words
            .into_iter()
            .map(|word| word.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let (graph, mut freqs): (Vec<HashSet<usize>>, Vec<i8>) = {
            let mut freqs: Vec<i8> = vec![-1; 26];
            for chs in &chses {
                for &ch in chs {
                    freqs[ch as usize - 'a' as usize] = 0;
                }
            }
            let mut graph: Vec<HashSet<usize>> = vec![HashSet::with_capacity(26); 26];
            for idx_w in 1..len_ws {
                let chs_prev = &chses[idx_w - 1];
                let chs_cur = &chses[idx_w];
                let len_prev = chs_prev.len();
                let len_cur = chs_cur.len();
                let len = std::cmp::max(len_prev, len_cur);
                for idx_ch in 0..len {
                    if idx_ch == len_prev {
                        break;
                    } else if idx_ch == len_cur {
                        return "".to_owned();
                    } else {
                        let ch_prev = chs_prev[idx_ch];
                        let ch_cur = chs_cur[idx_ch];
                        if ch_prev == ch_cur {
                            continue;
                        }
                        let idx_ch_prev: usize = ch_prev as usize - 'a' as usize;
                        let idx_ch_cur: usize = ch_cur as usize - 'a' as usize;
                        if graph[idx_ch_prev].insert(idx_ch_cur) {
                            freqs[idx_ch_cur] += 1;
                        }
                        break;
                    }
                }
            }
            (graph, freqs)
        };
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(26);
            for (idx, &freq) in freqs.iter().enumerate() {
                if freq == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        let mut ans = String::with_capacity(26);
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    let ch = (cur as u8 + 'a' as u8) as char;
                    ans.push(ch);
                    for &nxt in &graph[cur] {
                        freqs[nxt] -= 1;
                        if freqs[nxt] == 0 {
                            queue.push_back(nxt);
                        }
                    }
                }
            }
        }
        for freq in freqs {
            if freq > 0 {
                return "".to_owned();
            }
        }
        return ans;
    }
}
