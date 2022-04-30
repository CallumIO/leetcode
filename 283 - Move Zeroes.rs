// https://leetcode.com/problems/move-zeroes/
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = Vec::new();
        nums.retain(|x| {
            if *x != 0 {
                true
            } else {
                i.push(0);
                false
            }
        });
        &nums.append(&mut i);
    }
}
