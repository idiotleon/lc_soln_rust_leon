/// https://leetcode.com/problems/walls-and-gates/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/walls-and-gates/discuss/72748/Benchmarks-of-DFS-and-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let len_r: usize = rooms.len();
        let len_c: usize = rooms[0].len();
        const GATE: i32 = 0;
        for r in 0..len_r{
            for c in 0..len_c{
                if rooms[r][c] == GATE{
                    Self::dfs((r, c), rooms);
                }
            }
        }
    }
    fn dfs(coord: (usize, usize), rooms: &mut Vec<Vec<i32>>){
        let len_r: usize = rooms.len();
        let len_c: usize = rooms[0].len();
        let (r_cur, c_cur) = coord;
        for d in 0..4{
            let r_nxt: isize = r_cur as isize + Self::DIRS[d];
            let c_nxt: isize = c_cur as isize + Self::DIRS[d + 1];
            if r_nxt < 0 || c_nxt < 0 || r_nxt as usize >= len_r || c_nxt as usize >= len_c {
                continue;
            }
            let r_nxt: usize = r_nxt as usize;
            let c_nxt: usize = c_nxt as usize;
            if rooms[r_nxt][c_nxt] > 1 + rooms[r_cur][c_cur]{
                rooms[r_nxt][c_nxt] = 1 + rooms[r_cur][c_cur];
                Self::dfs((r_nxt, c_nxt), rooms);
            }
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
