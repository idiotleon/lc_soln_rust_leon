/// https://leetcode.com/problems/n-queens-ii/
///
/// Time Complexity:    O(`n`!)
/// Space Complexity:   O(`n`)
///
/// Reference:
/// https://leetcode.com/problems/n-queens-ii/discuss/298639/Rust-DFS-and-BitWis-0ms-Solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::backtrack(0, 0, 0, 0, n)
    }

    fn backtrack(row: i32, col: i32, pie: i32, na: i32, n: i32) -> i32 {
        if row >= n {
            return 1;
        }
        let mut cnt: i32 = 0;
        let mut bit_mask = (!(col | pie | na)) & ((1 << n) - 1);
        while bit_mask != 0 {
            let last_bit = bit_mask & -bit_mask;
            cnt += Self::backtrack(
                row + 1,
                col | last_bit,
                (pie | last_bit) << 1,
                (na | last_bit) >> 1,
                n,
            );
            bit_mask = bit_mask & (bit_mask - 1);
        }
        cnt
    }
}
