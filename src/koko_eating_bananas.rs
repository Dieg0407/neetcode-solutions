#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max = *piles.iter().max().unwrap();

        let mut start = 1;
        let mut end = max;
        let mut min_valid_value = end;

        while start <= end {
            let middle = (start + end) / 2;
            let time = total_time(&piles, &middle);

            if time <= h as i64 {
                min_valid_value = std::cmp::min(min_valid_value, middle);
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }

        min_valid_value
    }
}

fn total_time(piles: &Vec<i32>, k: &i32) -> i64 {
    let mut total: i64 = 0;
    for i in piles.iter() {
        if *i < *k {
            total += 1;
            continue;
        }
        total += *i as i64 / *k as i64;
        if *i % *k != 0 {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example01() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn example02() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);
    }

    #[test]
    fn example03() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }

    #[test]
    fn example04() {
        let piles = vec![312884470];
        let h = 312884469;
        assert_eq!(Solution::min_eating_speed(piles, h), 2);
    }

    #[test]
    fn example05() {
        let piles = vec![805306368, 805306368, 805306368];
        let h = 1000000000;
        assert_eq!(Solution::min_eating_speed(piles, h), 3);
    }
}
