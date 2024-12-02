pub mod day2 {
    use std::fs;

    pub fn run() {
        let contents = fs::read_to_string("input/day2").expect("File not found");

        let reports: Vec<Vec<i32>> = contents
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().expect("Invalid number"))
                    .collect()
            })
            .collect();

        println!("Total valid reports is: {}", count_valid_reports(&reports));
    }

    fn count_valid_reports(reports: &Vec<Vec<i32>>) -> i32 {
        reports
            .iter()
            .filter(|data| {
                (is_decreasing(&data) || is_increasing(&data)) && is_within_limits(&data)
            })
            .count() as i32
    }

    fn is_decreasing(data: &Vec<i32>) -> bool {
        data.windows(2).all(|pair| pair[0] > pair[1])
    }

    fn is_increasing(data: &Vec<i32>) -> bool {
        data.windows(2).all(|pair| pair[0] < pair[1])
    }

    fn is_within_limits(data: &Vec<i32>) -> bool {
        data.windows(2)
            .all(|pair| (1..=3).contains(&(pair[0] - pair[1]).abs()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        static TEST_DATA: [[i32; 5]; 6] = [
            [9, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ];

        #[test]
        fn test_is_decreasing() {
            let expected = vec![true, false, true, false, false, false];

            for (index, expected) in expected.iter().enumerate() {
                assert_eq!(
                    is_decreasing(&TEST_DATA[index].to_vec()),
                    *expected,
                    "testing iteration {}",
                    index
                );
            }
        }

        #[test]
        fn test_is_increasing() {
            let expected = vec![false, true, false, false, false, true];

            for (index, expected) in expected.iter().enumerate() {
                assert_eq!(
                    is_increasing(&TEST_DATA[index].to_vec()),
                    *expected,
                    "testing iteration {}",
                    index
                );
            }
        }

        #[test]
        fn test_is_within_limits() {
            let expected = vec![true, false, false, true, false, true];

            for (index, expected) in expected.iter().enumerate() {
                assert_eq!(
                    is_within_limits(&TEST_DATA[index].to_vec()),
                    *expected,
                    "testing iteration {}",
                    index
                );
            }
        }
    }
}
