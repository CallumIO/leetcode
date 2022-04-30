// https://leetcode.com/problems/contains-duplicate/
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut xi = 0;
        for i in &nums {
            let mut xj = 0;
            for j in &nums {
                if i == j && xj != xi {
                    return true;
                }
                xj += 1;
            }
            xi += 1;
        }
        false
    }
}
