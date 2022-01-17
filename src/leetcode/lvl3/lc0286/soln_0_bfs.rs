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
        let len_r: usize = rooms.len();
        let len_c: usize = rooms[0].len();
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const GATE: i32 = 0;
        const WALL: i32 = -1;
        const INF: i32 = i32::MAX;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for r in 0..len_r {
            for c in 0..len_c {
                if rooms[r][c] == GATE {
                    queue.push_back((r, c));
                }
            }
        }
        let mut steps: i32 = 1;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r_cur, c_cur)) = queue.pop_front() {
                    for d in 0..4 as usize {
                        let r_nxt: isize = r_cur as isize + DIRS[d];
                        let c_nxt: isize = c_cur as isize + DIRS[d + 1];
                        if r_nxt < 0
                            || c_nxt < 0
                            || r_nxt as usize >= len_r
                            || c_nxt as usize >= len_c
                        {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if rooms[r_nxt][c_nxt] != INF {
                            continue;
                        }
                        rooms[r_nxt][c_nxt] = steps;
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
            steps += 1;
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
