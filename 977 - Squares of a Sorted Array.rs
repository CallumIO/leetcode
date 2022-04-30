//https://leetcode.com/problems/squares-of-a-sorted-array/
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = vec![0; nums.len()];
        let mut h: usize = nums.len() - 1usize;
        let mut l: usize = 0usize;
        let mut x = nums.len() - 1;
        loop {
            if nums[h] + nums[l] <= 0 {
                r[x] = nums[l] * nums[l];
                l += 1usize;
            } else {
                r[x] = nums[h] * nums[h];
                if h == 0 {
                    break;
                }
                h -= 1usize;
            }
            if x == 0 {
                break;
            }
            x -= 1;
        }
        return r;
    }
}
