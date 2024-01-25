use std::usize;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        if t.len() == 1 {
            return vec![0];
        }
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut result = t.iter().map(|_| 0).collect::<Vec<i32>>();

        for i in 0..t.len() {
            if stack.is_empty() {
                stack.push((t[i], i));
                continue;
            }
            if stack.last().unwrap().0 >= t[i] {
                stack.push((t[i], i));
                continue;
            }

            while !stack.is_empty() && stack.last().unwrap().0 < t[i] {
                let value = stack.pop().unwrap();
                result[value.1] = (i - value.1) as i32;
            }

            stack.push((t[i], i));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example01() {
        let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            Solution::daily_temperatures(t),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example02() {
        let t = vec![30, 40, 50, 60];
        assert_eq!(Solution::daily_temperatures(t), vec![1, 1, 1, 0]);
    }

    #[test]
    fn example03() {
        let t = vec![30, 60, 90];
        assert_eq!(Solution::daily_temperatures(t), vec![1, 1, 0]);
    }
}
