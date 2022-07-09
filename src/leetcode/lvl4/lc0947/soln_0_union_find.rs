/// @author: Leon
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
/// Time Complexity:    O(`len_sts` ^ 2)
/// Space Complexity:   O(`RANGE`)
/// Reference:
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/197693/Java-Union-Find/201840
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/197693/Java-Union-Find
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/197668/Count-the-Number-of-Islands-O(N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let len_sts: usize = stones.len();
        const RANGE: usize = 1e3 as usize + 1;
        let mut roots: Vec<usize> = {
            let mut roots: Vec<usize> = vec![0; len_sts];
            for idx in 0..len_sts {
                roots[idx] = idx;
            }
            roots
        };
        let mut ranks: Vec<u16> = vec![1; len_sts];
        let mut isolated: u16 = len_sts as u16;
        for lo in 0..len_sts {
            for hi in lo + 1..len_sts {
                if stones[lo][0] == stones[hi][0] || stones[lo][1] == stones[hi][1] {
                    Self::union(lo, hi, &mut roots, &mut ranks, &mut isolated);
                }
            }
        }
        len_sts as i32 - isolated as i32
    }
    fn union(x: usize, y: usize, roots: &mut Vec<usize>, ranks: &mut Vec<u16>, isolated: &mut u16) {
        let root_x: usize = Self::find(x, roots);
        let root_y: usize = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if ranks[root_x] > ranks[root_y] {
            roots[root_y] = root_x;
            ranks[root_x] += 1;
        } else {
            roots[root_x] = root_y;
            ranks[root_y] += 1;
        }
        *isolated -= 1;
    }
    fn find(mut x: usize, roots: &mut Vec<usize>) -> usize {
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }
}
