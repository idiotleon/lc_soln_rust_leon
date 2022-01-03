/// @author: Leon
/// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
/// Time Complexity:    O(`len_r` * `len_c` + `len_c` * lg(`len_c`))
/// Space Complexity:   O(`len_c`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        const ONE: i32 = 1;
        let len_r: usize = mat.len();
        let _len_c: usize = mat[0].len();
        let sorted: Vec<(u8, usize)> = {
            let mut tmp: Vec<(u8, usize)> = vec![(0, 0); len_r];
            for (idx_r, row) in mat.into_iter().enumerate() {
                let mut cnt: u8 = 0;
                for (_idx_c, value) in row.into_iter().enumerate() {
                    if value == ONE {
                        cnt += 1;
                    }
                }
                tmp[idx_r] = (cnt, idx_r);
            }
            tmp.sort_by(|&a, &b| {
                let cmp_first = a.0.cmp(&b.0);
                let cmp_sec = a.1.cmp(&b.1);
                if cmp_first != std::cmp::Ordering::Equal {
                    cmp_first
                } else {
                    cmp_sec
                }
            });
            tmp
        };
        let ans: Vec<i32> = {
            let mut tmp: Vec<i32> = Vec::new();
            for idx in 0..k as usize {
                tmp.push(sorted[idx].1 as i32);
            }
            tmp
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let k: i32 = 2;
        let actual = Solution::k_weakest_rows(mat, k);
        let expected = vec![0, 2];
        assert_eq!(expected, actual);
    }
}
