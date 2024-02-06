#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        recursive_min_array(nums)
    }
}

pub fn recursive_min_array(numbers: Vec<i32>) -> i32 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    if numbers.len() == 2 {
        return std::cmp::min(numbers[0], numbers[1]);
    }

    let middle = numbers.len() / 2;
    return if numbers[middle] > numbers[numbers.len() - 1] {
        recursive_min_array(numbers[(middle + 1)..].to_vec())
    } else if numbers[middle] < numbers[middle - 1] {
        numbers[middle]
    } else {
        recursive_min_array(numbers[..middle].to_vec())
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn example02() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn example03() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    }
}
