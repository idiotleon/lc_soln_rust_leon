/// @author: Leon
/// https://leetcode.com/problems/sort-even-and-odd-indices-independently/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
/// Reference:
/// https://leetcode.com/problems/sort-even-and-odd-indices-independently/discuss/1748447/Heap(O(n-log-n))-and-Counting-Sort(O(n))-in-Java
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        const RANGE: u8 = 100 + 1;
        let (mut odd, mut even): (Vec<i32>, Vec<i32>) = {
            let mut odd: Vec<i32> = vec![0; RANGE as usize];
            let mut even: Vec<i32> = vec![0; RANGE as usize];
            for (idx, num) in nums.into_iter().enumerate() {
                if idx % 2 == 0 {
                    even[num as usize] += 1;
                } else {
                    odd[num as usize] += 1;
                }
            }
            (odd, even)
        };
        let mut o: usize = RANGE as usize - 1;
        let mut e: usize = 0;
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = vec![0; len_n];
            for idx in 0..len_n {
                if idx % 2 == 0 {
                    while even[e] == 0 {
                        e += 1;
                    }
                    res[idx] = e as i32;
                    even[e] -= 1;
                } else {
                    while odd[o] == 0 {
                        o -= 1;
                    }
                    res[idx] = o as i32;
                    odd[o] -= 1;
                }
            }
            res
        };
        ans
    }
}
