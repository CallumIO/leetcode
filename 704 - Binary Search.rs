// https://leetcode.com/problems/binary-search/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0i32; // lo
        let mut p = nums.len() - 1; // hi
        let mut m; // mid
        while i <= p as i32 {
            m = i + (p as i32 - i) / 2;
            let m_i = m as usize;
            let j = &nums[m_i];
            if *j == target {
                return m_i as i32;
            }
            if *j < target {
                i = m + 1;
            } else {
                p = (m - 1) as usize;
            }
        }
        return -1i32;
    }
}
