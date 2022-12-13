/// @author: Leon
/// https://leetcode.com/problems/minimum-falling-path-sum/description/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Time Complexity:    O(1)`
/// Reference:
/// https://leetcode.com/problems/minimum-falling-path-sum/discuss/186666/C%2B%2BJava-4-lines-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_falling_path_sum(mut mat: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = mat.len();
        let len_cs: usize = mat[0].len();
        for r in 1..len_rs {
            for c in 0..len_cs {
                mat[r][c] += std::cmp::min(
                    mat[r - 1][if c > 0 { c - 1 } else { 0 }],
                    std::cmp::min(
                        mat[r - 1][c],
                        if c + 1 < len_cs {
                            mat[r - 1][c + 1]
                        } else {
                            mat[r - 1][len_cs - 1]
                        },
                    ),
                );
            }
        }
        return *mat[len_rs - 1].iter().min().unwrap();
    }
}
