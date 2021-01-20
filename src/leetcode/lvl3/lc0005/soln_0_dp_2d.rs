impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        
        let mut idx_start = 0;
        let mut idx_end = 0;
        let chs: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![false; chs.len()]; chs.len()];
        
        for lo in (0..chs.len()).rev(){
            for hi in (lo..chs.len()){
                dp[lo][hi] = (chs[lo] == chs[hi]) && (hi - lo < 2 || dp[lo + 1][hi - 1]);
                
                if dp[lo][hi] && (hi - lo > idx_end - idx_start){
                    idx_start = lo;
                    idx_end = hi;
                }
            }
        }
        
        return chs[idx_start..idx_end + 1].iter().collect::<String>();
    }
}