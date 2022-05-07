use std::collections::{HashSet, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/minimum-knight-moves/
/// Time Complexity:    O(V + E) ~
/// Space Complexity:   O(V + E) ~
/// Reference:
/// https://leetcode.com/problems/minimum-knight-moves/discuss/947138/Python-3-or-BFS-DFS-Math-or-Explanation
/// https://leetcode.com/problems/minimum-knight-moves/discuss/387071/JavaPython-3-BFS-code-using-symmetry
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [(i32, i32); 8] = &[
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
        (1, 2),
        (2, 1),
    ];

    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((0, 0));

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));

        let mut moves: i32 = 0;

        while !queue.is_empty() {
            let len_q = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    if r == x && c == y {
                        return moves;
                    }

                    for (dr, dc) in Self::DIRS {
                        let nxt = (r + dr, c + dc);
                        if visited.insert(nxt) {
                            queue.push_back(nxt);
                        }
                    }
                }
            }
            moves += 1;
        }
        -1
    }
}
