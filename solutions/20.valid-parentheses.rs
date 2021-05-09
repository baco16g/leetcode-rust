/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut tokens = s.chars().collect::<String>();
        while let Some(token) = tokens.pop() {
            if let Some(l) = stack.last() {
                match token {
                    '(' if *l == ')' => {
                        stack.remove(stack.len() - 1);
                    }
                    '[' if *l == ']' => {
                        stack.remove(stack.len() - 1);
                    }
                    '{' if *l == '}' => {
                        stack.remove(stack.len() - 1);
                    }
                    ')' | ']' | '}' => {
                        stack.push(token);
                    }
                    _ => return false,
                }
            } else {
                stack.push(token);
            }
        }
        stack.is_empty()
    }
}
// @lc code=end
