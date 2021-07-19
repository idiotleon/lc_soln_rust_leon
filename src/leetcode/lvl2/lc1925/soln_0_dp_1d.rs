/// https://leetcode.com/problems/count-square-sum-triples/
/// 
/// Time Complexity:    O(`n` ^ 2)
/// Space Complexity:   O(`n` ^ 2)
/// 
/// Reference:
/// https://leetcode.com/problems/count-square-sum-triples/discuss/1329149/O(n-*-n)/1005150
/// https://leetcode.com/problems/count-square-sum-triples/discuss/1329149/O(n-*-n)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let n: usize = n as usize;
        let freqs: Vec<i32> = {
            let mut tmp = vec![0; n * n + 1];
            for i in 1..=n{
                tmp[i * i] = 1;
            }
            tmp
        };
        
        let mut cnt = 0;
        for a in 1..=n{
            let mut b = 1;
            while a * a + b * b <= n * n{
                cnt += freqs[a * a + b * b];
                
                b += 1;
            }
        }
        
        cnt
    }
}