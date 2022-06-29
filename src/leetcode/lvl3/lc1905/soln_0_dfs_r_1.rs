/// @author: Leon
/// https://leetcode.com/problems/count-sub-islands/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/count-sub-islands/discuss/1284232/Fill-Second-and-Count-First
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const ISLAND: i32 = 1;
    const WATER: i32 = 0;
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid2.len();
        let len_cs: usize = grid2[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut cnt: i32 = 0;
        for r in 0..len_rs {
            for c in 0..len_cs {
                let (cnt1, cnt2) =
                    Self::dfs((r as isize, c as isize), &mut visited, &grid1, &grid2);
                if cnt1 > 0 && cnt1 == cnt2 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn dfs(
        coord: (isize, isize),
        visited: &mut Vec<Vec<bool>>,
        grid1: &Vec<Vec<i32>>,
        grid2: &Vec<Vec<i32>>,
    ) -> (i32, i32) {
        let len_rs: isize = grid2.len() as isize;
        let len_cs: isize = grid2[0].len() as isize;
        let (r, c) = coord;
        if r < 0
            || c < 0
            || r >= len_rs
            || c >= len_cs
            || visited[r as usize][c as usize]
            || grid2[r as usize][c as usize] == Self::WATER
        {
            return (0, 0);
        }
        visited[r as usize][c as usize] = true;
        let mut cnt1: i32 = grid1[r as usize][c as usize];
        let mut cnt2: i32 = 1;
        for d in 0..4 {
            let r_nxt: isize = r + Self::DIRS[d];
            let c_nxt: isize = c + Self::DIRS[d + 1];
            let (res1, res2) = Self::dfs((r_nxt, c_nxt), visited, grid1, grid2);
            cnt1 += res1;
            cnt2 += res2;
        }
        (cnt1, cnt2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let grid1 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ];
        let grid2 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];
        let actual = Solution::count_sub_islands(grid1, grid2);
        let expected = 3;
        assert_eq!(expected, actual);
    }
}
