/// https://leetcode.com/problems/image-smoother/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(1) / O(`len_r` * `len_c`)
/// Referenece:
/// https://leetcode.com/problems/image-smoother/discuss/106602/Very-Clean-Solution-in-Java
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_r: usize = img.len();
        let len_c: usize = img[0].len();
        let ans: Vec<Vec<i32>> = {
            let mut res: Vec<Vec<i32>> = vec![vec![0; len_c]; len_r];
            for (idx_r, row) in img.iter().enumerate() {
                for (idx_c, &_value) in row.iter().enumerate() {
                    let mut cnt: u8 = 0;
                    let mut sum: u16 = 0;
                    for delta_r in vec![-1, 0, 1] {
                        for delta_c in vec![-1, 0, 1] {
                            let r: isize = idx_r as isize + delta_r;
                            let c: isize = idx_c as isize + delta_c;
                            if r < 0 || c < 0 || r as usize >= len_r || c as usize >= len_c {
                                continue;
                            }
                            cnt += 1;
                            sum += img[r as usize][c as usize] as u16;
                        }
                    }
                    res[idx_r][idx_c] = sum as i32 / cnt as i32;
                }
            }
            res
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let img: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let actual = Solution::image_smoother(img);
        let expected = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_2() {
        let img: Vec<Vec<i32>> = vec![
            vec![2, 3, 4],
            vec![5, 6, 7],
            vec![8, 9, 10],
            vec![11, 12, 13],
            vec![14, 15, 16],
        ];
        let actual = Solution::image_smoother(img);
        let expected = vec![
            vec![4, 4, 5],
            vec![5, 6, 6],
            vec![8, 9, 9],
            vec![11, 12, 12],
            vec![13, 13, 14],
        ];
        assert_eq!(expected, actual);
    }
}
