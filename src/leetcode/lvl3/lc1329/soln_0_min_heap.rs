use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// @author: Leon
/// https://leetcode.com/problems/sort-the-matrix-diagonally/
/// Time Complexity:    O(`len_rs` * `len_cs` * lg(`len_rs` * `len_cs`))
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/sort-the-matrix-diagonally/discuss/489749/JavaPython-Straight-Forward
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_rs: usize = mat.len();
        let len_cs: usize = mat[0].len();
        let mut map: HashMap<usize, BinaryHeap<Reverse<i32>>> =
            HashMap::with_capacity(len_rs * len_cs);
        for r in 0..len_rs {
            for c in 0..len_cs {
                map.entry(r - c).or_default().push(Reverse(mat[r][c]));
            }
        }
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_cs]; len_rs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                if let Some(heap) = map.get_mut(&(r - c)) {
                    if let Some(Reverse(top)) = heap.pop() {
                        ans[r][c] = top;
                    }
                }
            }
        }
        return ans;
    }
}
