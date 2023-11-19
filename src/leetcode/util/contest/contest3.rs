struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len_hts: usize = heights.len();
        let len_qs: usize = queries.len();
        let indices: Vec<Vec<usize>> = {
            let mut res: Vec<Vec<usize>> = vec![Vec::with_capacity(len_hts); len_hts];
            for lo in 0..len_hts - 1 {
                for hi in lo + 1..len_hts {
                    if heights[lo] < heights[hi] {
                        res[lo].push(hi);
                    }
                }
            }
            res
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_qs);
        for query in queries {
            let q1: usize = query[0] as usize;
            let q2: usize = query[1] as usize;
            if q1 == q2 {
                ans.push(q1 as i32);
            } else {
                let res: i32 = Self::get_res(q1, q2, &indices, &heights);
                ans.push(res);
            }
        }
        return ans;
    }
    fn get_res(q1: usize, q2: usize, indices: &Vec<Vec<usize>>, heights: &Vec<i32>) -> i32 {
        if heights[q1] < heights[q2] {
            if Self::contains(&indices[q1], q2) {
                return q2 as i32;
            }
        } else if heights[q1] > heights[q2] {
            if Self::contains(&indices[q2], q1) {
                return q1 as i32;
            }
        }
        return Self::find_common(&indices[q1], &indices[q2]);
    }
    fn find_common(indices1: &Vec<usize>, indices2: &Vec<usize>) -> i32 {
        let len1: usize = indices1.len();
        let len2: usize = indices2.len();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        while idx1 < len1 && idx2 < len2 {
            if indices1[idx1] == indices2[idx2] {
                return indices1[idx1] as i32;
            }
            if indices1[idx1] > indices2[idx2] {
                idx2 += 1;
            } else {
                idx1 += 1;
            }
        }
        return -1;
    }
    fn contains(indices1: &Vec<usize>, q2: usize) -> bool {
        for &idx in indices1 {
            if idx == q2 {
                return true;
            }
        }
        return false;
    }
}

#[allow(dead_code, unused_imports)]
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let heights: Vec<i32> = vec![6, 4, 8, 5, 2, 7];
        let queries: Vec<Vec<i32>> =
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
        let expected: Vec<i32> = vec![2, 5, -1, 5, 2];
        let actual: Vec<i32> = Solution::leftmost_building_queries(heights, queries);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let heights: Vec<i32> = vec![5, 3, 8, 2, 6, 1, 4, 6];
        let queries: Vec<Vec<i32>> =
            vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]];
        let expected: Vec<i32> = vec![7, 6, -1, 4, 6];
        let actual: Vec<i32> = Solution::leftmost_building_queries(heights, queries);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {}
    #[test]
    fn it_works_with_sample_input_4() {}
}
