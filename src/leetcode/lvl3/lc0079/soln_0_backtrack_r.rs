/// @author: Leon
/// https://leetcode.com/problems/word-search/
/// Time Complexity:    O(((`len_r` * `len_c`) ^ 2) * (3 ^ `len_w`))
/// Space Complexity:   O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/word-search/discuss/27795/What-is-the-time-complexity-I-think-it-is-O(N2-*-4k)-where-k-is-the-length-of-word/181405
/// https://leetcode.com/problems/word-search/discuss/27795/What-is-the-time-complexity-I-think-it-is-O(N2-*-4k)-where-k-is-the-length-of-word
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let len_r: usize = board.len();
        let len_c: usize = board[0].len();
        let chs: Vec<char> = word.chars().collect();
        for r in 0..len_r {
            for c in 0..len_c {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];
                if Self::backtrack((r as isize, c as isize), 0, &chs, &mut visited, &board) {
                    return true;
                }
            }
        }
        false
    }
    fn backtrack(
        coord: (isize, isize),
        idx_ch: usize,
        chs: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        board: &Vec<Vec<char>>,
    ) -> bool {
        let len_w: usize = chs.len();
        if idx_ch == len_w {
            return true;
        }
        let len_r: usize = board.len();
        let len_c: usize = board[0].len();
        let (r_cur, c_cur) = coord;
        if r_cur < 0
            || r_cur as usize >= len_r
            || c_cur < 0
            || c_cur as usize >= len_c
            || visited[r_cur as usize][c_cur as usize]
            || board[r_cur as usize][c_cur as usize] != chs[idx_ch]
        {
            return false;
        }
        visited[r_cur as usize][c_cur as usize] = true;
        for d in 0..4 {
            let r_nxt: isize = r_cur + Self::DIRS[d];
            let c_nxt: isize = c_cur + Self::DIRS[d + 1];
            if Self::backtrack((r_nxt, c_nxt), idx_ch + 1, chs, visited, board) {
                return true;
            };
        }
        visited[r_cur as usize][c_cur as usize] = false;
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word: String = "ABCCED".to_owned();
        let actual = Solution::exist(board, word);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_82() {
        let board: Vec<Vec<char>> = vec![vec!['a']];
        let word: String = "a".to_owned();
        let actual = Solution::exist(board, word);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
