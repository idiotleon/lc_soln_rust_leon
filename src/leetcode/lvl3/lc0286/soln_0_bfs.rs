use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/walls-and-gates/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/walls-and-gates/discuss/72748/Benchmarks-of-DFS-and-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const INF: i32 = i32::MAX;
        // const INF: i32 = 2147483647;
        const GATE: i32 = 0;
        const WALL: i32 = -1;
        let len_rs: usize = rooms.len();
        let len_cs: usize = rooms[0].len();
        let mut queue: VecDeque<(usize, usize)> = {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_rs * len_cs);
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if rooms[r][c] == GATE {
                        queue.push_back((r, c));
                    }
                }
            }
            queue
        };
        let mut step: i32 = 1;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    for d in 0..4 {
                        let r_nxt: isize = r as isize + DIRS[d];
                        let c_nxt: isize = c as isize + DIRS[d + 1];
                        if r_nxt < 0 || c_nxt < 0 {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if r_nxt >= len_rs
                            || c_nxt >= len_cs
                            || rooms[r_nxt][c_nxt] == WALL
                            || rooms[r_nxt][c_nxt] != INF
                        {
                            continue;
                        }
                        rooms[r_nxt][c_nxt] = step;
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
            step += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mut rooms: Vec<Vec<i32>> = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        Solution::walls_and_gates(&mut rooms);
        let expected: Vec<Vec<i32>> = vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ];
        assert_eq!(expected, rooms);
    }
}
