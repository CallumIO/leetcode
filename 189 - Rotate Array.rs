//https://leetcode.com/problems/rotate-array/
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let (k, mut i, mut s) = (k as usize % n, 0, 0);
        while i < n {
            let (mut j, mut p) = (s, nums[s]);
            loop {
                let m = (j + k) % n;
                let t = nums[m];
                nums[m] = p;
                p = t;
                j = m;
                i += 1;
                if s == j {
                    break;
                }
            }
            s += 1;
        }
    }
}
