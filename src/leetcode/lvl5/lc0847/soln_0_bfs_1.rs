use std::collections::{HashSet, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
/// Reference:
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/135809/Fast-BFS-Solution-(46ms)-Clear-Detailed-Explanation-Included/237938
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/135809/Fast-BFS-Solution-(46ms)-Clear-Detailed-Explanation-Included
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let len_n: usize = graph.len();
        let full_mask: u16 = (1 << len_n) - 1;
        let mut seen: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<State> = {
            let mut queue: VecDeque<State> = VecDeque::new();
            for idx in 0..len_n {
                let state = State {
                    node: idx as u8,
                    bit_mask: 1 << idx,
                };
                let hash = &state.to_hash();
                queue.push_back(state);
                seen.insert(hash.to_owned());
            }
            queue
        };
        let mut lvl: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(State {
                    node: cur,
                    bit_mask,
                }) = queue.pop_front()
                {
                    if bit_mask == full_mask {
                        return lvl;
                    }
                    for &nxt in graph[cur as usize].iter() {
                        let nxt_state = State {
                            node: nxt as u8,
                            bit_mask: bit_mask | (1 << nxt),
                        };
                        let hash = &nxt_state.to_hash();
                        if seen.insert(hash.to_owned()) {
                            queue.push_back(nxt_state);
                        };
                    }
                }
            }
            lvl += 1;
        }
        lvl
    }
}

struct State {
    node: u8,
    bit_mask: u16,
}

impl State {
    fn to_hash(&self) -> String {
        const SPLITTER: char = '#';
        let mut hash: String = self.node.to_string();
        hash.push(SPLITTER);
        hash.push_str(&self.bit_mask.to_string());
        hash
    }
}
