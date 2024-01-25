use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut count = HashSet::new();
        Solution::generate(n, 0, 0, "".to_string(), &mut count);

        count.into_iter().collect()
    }

    fn generate(total: i32, left: i32, right: i32, value: String, count: &mut HashSet<String>) {
        if left == total && right == total {
            count.insert(value.clone());
            return;
        }

        if left < total {
            Solution::generate(total, left + 1, right, format!("{}(", value), count);
        }

        if right < left {
            value.clone().push(')');
            Solution::generate(total, left, right + 1, format!("{})", value), count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example02() {
        let n = 2;
        let mut expected = vec!["(())".to_string(), "()()".to_string()];
        expected.sort();
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example03() {
        let n = 3;
        let mut expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        expected.sort();

        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example01() {
        let n = 1;
        let expected = vec!["()".to_string()];
        assert_eq!(Solution::generate_parenthesis(n), expected);
    }

    #[test]
    fn example04() {
        let n = 4;
        let mut expected = [
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        expected.sort();
        let mut result = Solution::generate_parenthesis(n);
        result.sort();
        assert_eq!(result, expected);
    }
}
