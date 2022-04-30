// https://leetcode.com/problems/reverse-string/
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut lp, mut rp) = (0, s.len() - 1);
        while lp < rp {
            let mut b: char = s[lp];
            s[lp] = s[rp];
            s[rp] = b;
            lp += 1;
            rp -= 1;
        }
    }
}
