/// @author: Leon
/// https://leetcode.com/problems/special-positions-in-a-binary-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` + `len_cs`) ~ O(max(`len_rs`, `len_cs`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = mat.len();
        let len_cs: usize = mat[0].len();
        let (freq_r, freq_c): (Vec<u8>, Vec<u8>) = {
            let mut freq_r: Vec<u8> = vec![0; len_rs];
            let mut freq_c: Vec<u8> = vec![0; len_cs];
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if mat[r][c] == 1 {
                        freq_r[r] += 1;
                        freq_c[c] += 1;
                    }
                }
            }
            (freq_r, freq_c)
        };
        let mut cnt: i32 = 0;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if mat[r][c] == 1 {
                    if freq_r[r] == 1 && freq_c[c] == 1 {
                        cnt += 1;
                    }
                }
            }
        }
        return cnt;
    }
}
