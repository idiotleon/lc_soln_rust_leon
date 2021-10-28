/// https://leetcode.com/problems/n-queens-ii/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/n-queens-ii/discuss/1238496/Rust-backtracking-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut row: Vec<i32> = Vec::new();
        Self::backtrack(&mut row, n)
    }

    fn backtrack(mut row: &mut Vec<i32>, n: i32) -> i32 {
        let len_c: usize = row.len();
        if len_c == n as usize {
            return 1;
        }
        let mut cnt: i32 = 0;
        for c1 in 0..n {
            if row
                .iter()
                .enumerate()
                .any(|(r, &c2)| c2 == c1 || (len_c - r) as i32 == (c2 - c1).abs())
            {
                continue;
            }
            row.push(c1);
            cnt += Self::backtrack(&mut row, n);
            row.pop();
        }
        cnt
    }
}
