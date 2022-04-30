// https://leetcode.com/problems/first-bad-version/
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut i = 0i32;
        let mut p = n;
        let mut m;
        let mut r = 0i32;
        while i <= p {
            m = i + (p - i) / 2;
            if (self.isBadVersion(m)) {
                r = m;
                p = m - 1;
            } else {
                i = m + 1
            }
        }
        return r;
    }
}
