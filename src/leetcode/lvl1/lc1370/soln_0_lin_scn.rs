/// @author: Leon
/// https://leetcode.com/problems/increasing-decreasing-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_string(s: String) -> String {
        let len_s: usize = s.len();
        let mut cnt: usize = len_s;
        let mut freqs = {
            let mut freqs: Vec<u16> = vec![0; 26];
            for ch in s.chars(){
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut ans: String = "".to_owned();
        while cnt > 0{
            let seg_inc: String = Self::get_inc(&mut freqs, &mut cnt);
            ans.push_str(&seg_inc);
            let seg_dec: String = Self::get_dec(&mut freqs, &mut cnt);
            ans.push_str(&seg_dec);
        }
        return ans;
    }
    fn get_inc(freqs: &mut Vec<u16>, cnt: &mut usize) -> String{
        let len_cs: usize = freqs.len();
        let mut ans: String = "".to_owned();
        for idx in 0..len_cs{
            let freq = &mut freqs[idx];
            if *freq > 0{
                ans.push((idx as u8 + 'a' as u8) as char);
                *freq -= 1;
                *cnt -= 1;
            }
        }
        return ans;
    }
    fn get_dec(freqs: &mut Vec<u16>, cnt: &mut usize) -> String{
        let len_cs: usize = freqs.len();
        let mut ans: String = "".to_owned();
        for idx in (0..len_cs).rev(){
            let freq = &mut freqs[idx];
            if *freq > 0{
                ans.push((idx as u8 + 'a' as u8) as char);
                *freq -= 1;
                *cnt -= 1;
            }
        }
        return ans;
    }
}