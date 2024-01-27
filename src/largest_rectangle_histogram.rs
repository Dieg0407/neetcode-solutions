#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut max_area = 0;

        for i in 0..heights.len() {
            if stack.is_empty() {
                stack.push((heights[i], i));
                max_area = std::cmp::max(max_area, heights[i]);
                continue;
            }
            let e = stack.last().unwrap();
            if e.0 < heights[i] {
                stack.push((heights[i], i));
                continue;
            }
            let mut new_pos: usize = 0;
            while !stack.is_empty() && stack.last().unwrap().0 >= heights[i] {
                let e = stack.pop().unwrap();
                let area = e.0 * (i - e.1) as i32;
                max_area = std::cmp::max(max_area, area);
                new_pos = e.1;
            }
            stack.push((heights[i], new_pos));
        }

        while !stack.is_empty() {
            let e = stack.pop().unwrap();
            let area = e.0 * (heights.len() - e.1) as i32;
            max_area = std::cmp::max(max_area, area);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(Solution::largest_rectangle_area(heights), 10);
    }

    #[test]
    fn example2() {
        let heights = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(heights), 4);
    }
}
