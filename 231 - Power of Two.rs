// https://leetcode.com/problems/power-of-two/
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n == 0 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        return n == 1;
    }
}
