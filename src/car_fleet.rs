#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 0 {
            return 1;
        }

        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_by(|a, b| b.0.cmp(&a.0));

        let mut total = 1;
        let mut time = (target - cars[0].0) as f64 / cars[0].1 as f64;

        for i in 1..cars.len() {
            let car = cars[i];
            let distance = car.0 as f64 + car.1 as f64 * time;
            if distance >= target as f64 {
                continue;
            }
            total += 1;
            time = (target - car.0) as f64 / car.1 as f64;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        assert_eq!(Solution::car_fleet(target, position, speed), 3);
    }

    #[test]
    fn example02() {
        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        assert_eq!(Solution::car_fleet(target, position, speed), 1);
    }

    #[test]
    fn example03() {
        let target = 10;
        let position = vec![0, 4, 2];
        let speed = vec![2, 1, 3];
        assert_eq!(Solution::car_fleet(target, position, speed), 1);
    }
}
