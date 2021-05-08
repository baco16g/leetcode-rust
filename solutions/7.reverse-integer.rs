/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let caller = |n| {
            let mut input = n;
            let mut result = 0i32;
            while input != 0 {
                result = result.checked_mul(10)?.checked_add(input % 10)?;
                input = input / 10;
            }
            Some(result)
        };
        caller(x).unwrap_or(0)
    }
}
// @lc code=end
