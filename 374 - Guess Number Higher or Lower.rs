// https://leetcode.com/problems/guess-number-higher-or-lower/
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut i = 0i32;
        let mut j = n;
        loop {
            let k = i + (j - i) / 2;
            match guess(k) {
                0 => break k,
                -1 => j = k - 1,
                _ => i = k + 1,
            }
        }
    }
}
