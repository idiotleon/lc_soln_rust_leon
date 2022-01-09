use std::collections::BinaryHeap;
/// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
/// Time Complexity:    O(`len_r` * lg(`len_r`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let len_r: usize = mat.len();
        let _len_c: usize = mat[0].len();
        const ONE: i32 = 1;
        let row_to_ones: Vec<u8> = {
            let mut res: Vec<u8> = vec![0; len_r];
            for (idx, row) in mat.into_iter().enumerate(){
                let mut cnt: u8 = 0;
                for num in row{
                    if num == ONE{
                        cnt += 1;
                    }
                }
                res[idx] = cnt;
            }
            res
        };
        let mut heap: BinaryHeap<(u8, usize)> = BinaryHeap::new();
        for (idx, ones) in row_to_ones.into_iter().enumerate(){
            heap.push((ones, idx));
            if heap.len() > k as usize{
                heap.pop();
            }
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = vec![0; k as usize];
            let mut idx_rs: isize = k as isize - 1;
            while let Some((_ones, idx)) = heap.pop(){
                res[idx_rs as usize] = idx as i32;
                idx_rs -= 1;
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
    fn it_works_with_sample_input_2() {
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
