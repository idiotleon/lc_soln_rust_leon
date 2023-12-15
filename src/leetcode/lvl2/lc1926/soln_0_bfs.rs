use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(max(`len_rs`, `len_cs`))
/// Reference:
/// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let len_rs: usize = maze.len();
        let len_cs: usize = maze[0].len();
        const WALL: char = '+';
        const EMPTY: char = '.';
        const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
        let r_start: usize = entrance[0] as usize;
        let c_start: usize = entrance[1] as usize;
        let mut queue: VecDeque<(usize, usize)> = {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_rs * len_cs);
            queue.push_back((r_start, c_start));
            queue
        };
        let mut seen: HashSet<(usize, usize)> = {
            let mut seen: HashSet<(usize, usize)> = HashSet::with_capacity(len_rs * len_cs);
            seen.insert((r_start, c_start));
            seen
        };
        let mut step: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    if (r != r_start || c != c_start)
                        && (r == 0 || r == len_rs - 1 || c == 0 || c == len_cs - 1)
                    {
                        return step;
                    }
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
                            || maze[r_nxt][c_nxt] == WALL
                            || !seen.insert((r_nxt, c_nxt))
                        {
                            continue;
                        }
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
            step += 1;
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let maze: Vec<Vec<char>> = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance: Vec<i32> = vec![1, 2];
        let expected: i32 = 1;
        let actual: i32 = Solution::nearest_exit(maze, entrance);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let maze: Vec<Vec<char>> = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance: Vec<i32> = vec![1, 0];
        let expected: i32 = 2;
        let actual: i32 = Solution::nearest_exit(maze, entrance);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let maze: Vec<Vec<char>> = vec![vec!['.', '+']];
        let entrance: Vec<i32> = vec![0, 0];
        let expected: i32 = -1;
        let actual: i32 = Solution::nearest_exit(maze, entrance);
        assert_eq!(expected, actual);
    }
}
