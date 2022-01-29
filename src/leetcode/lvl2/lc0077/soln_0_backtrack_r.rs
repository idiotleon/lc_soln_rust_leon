/// @author: Leon
/// https://leetcode.com/problems/combinations/
/// Time Complexity:    O(`n`!)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(1, n, k as usize, &mut path, &mut paths);
        paths
    }
    fn backtrack(num_start: i32, n: i32, k: usize, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        // no need at all
        // if num_start > n + 1{
        //     return;
        // }
        if path.len() == k {
            paths.push(path.to_vec());
            return;
        }
        for num in num_start..=n {
            path.push(num);
            Self::backtrack(num + 1, n, k, path, paths);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 4;
        let k: i32 = 2;
        let actual: Vec<Vec<i32>> = {
            let mut res = Solution::combine(n, k);
            res.sort();
            res
        };
        let expected: Vec<Vec<i32>> = {
            let mut res = vec![
                vec![2, 4],
                vec![3, 4],
                vec![2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
            ];
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
}
