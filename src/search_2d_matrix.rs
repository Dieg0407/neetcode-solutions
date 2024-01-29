use std::{collections::HashSet, usize};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix[0][0] > target {
            return false;
        }
        if matrix[matrix.len() - 1][matrix[0].len() - 1] < target {
            return false;
        }

        let row_index = Solution::select_row(&matrix, &target);
        println!("row: {:?}", row_index);

        if row_index == -1 {
            return false;
        }

        let row = &matrix[row_index as usize];

        let mut start = 0;
        let mut end = row.len() as i32 - 1;
        loop {
            if start > end {
                break;
            }

            let middle = (start + end) / 2;
            let middle_value = row[middle as usize];

            if middle_value == target {
                return true;
            }

            if middle_value > target {
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }

        false
    }

    fn select_row(matrix: &Vec<Vec<i32>>, target: &i32) -> i32 {
        let mut visited_rows: HashSet<i32> = HashSet::new();
        let mut start = 0;
        let mut end = matrix.len() as i32 - 1;

        loop {
            let middle = (start + end) / 2;
            if visited_rows.contains(&middle) || start > end {
                return -1;
            }

            let middle_bottom: i32 = matrix[middle as usize][0];
            let middle_top: i32 = matrix[middle as usize][matrix[middle as usize].len() - 1];

            if middle_bottom <= *target && middle_top >= *target {
                return middle as i32;
            }
            visited_rows.insert(middle);
            if middle_bottom > *target {
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(Solution::search_matrix(matrix, target), true);
    }

    #[test]
    fn example02() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert_eq!(Solution::search_matrix(matrix, target), false);
    }
}
