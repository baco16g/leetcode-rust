/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let rev = x
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        x == rev
    }
}
// @lc code=end
