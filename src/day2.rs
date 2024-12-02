pub mod day2 {
    use std::cmp::max;
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
        println!(
            "Total kinda valid reports is: {}",
            count_kinda_valid_reports(&reports)
        );
    }

    fn count_valid_reports(reports: &Vec<Vec<i32>>) -> i32 {
        reports.iter().filter(|data| is_safe(data, 0)).count() as i32
    }

    fn count_kinda_valid_reports(reports: &Vec<Vec<i32>>) -> i32 {
        reports.iter().filter(|data| is_safe(data, 1)).count() as i32
    }

    fn is_safe(data: &Vec<i32>, tolerance: i32) -> bool {
        let tolerance = tolerance;

        let pairs = data.len() - 1;
        let count = get_increasing_decreasing_pairs_count(data);

        let difference = (pairs - count) as i32;

        difference <= tolerance && is_within_limits(data, max(0, tolerance - difference))
    }

    fn get_increasing_decreasing_pairs_count(data: &Vec<i32>) -> usize {
        max(
            data.windows(2).filter(|pair| pair[0] >= pair[1]).count(),
            data.windows(2).filter(|pair| pair[0] <= pair[1]).count(),
        )
    }

    fn is_within_limits(data: &Vec<i32>, tolerance: i32) -> bool {
        if let Some((first, rest)) = data.split_first() {
            let mut previous = first;
            let mut error_count = 0;

            for value in rest {
                let diff = (value - previous).abs();
                if ![1, 2, 3].contains(&diff) {
                    if error_count == tolerance {
                        return false;
                    } else {
                        error_count += 1;
                        continue;
                    }
                }

                previous = value;
            }

            return true;
        }

        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        static TEST_DATA: [[i32; 5]; 7] = [
            [9, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
            [1, 3, 2, 5, 12],
        ];

        #[test]
        fn test_is_perfectly_safe() {
            let expected = vec![true, false, false, false, false, true, false];

            for (index, expected) in expected.iter().enumerate() {
                assert_eq!(
                    is_safe(&TEST_DATA[index].to_vec(), 0),
                    *expected,
                    "testing iteration {}",
                    index
                );
            }
        }

        #[test]
        fn test_is_kinda_safe() {
            let expected = vec![true, false, false, true, true, true, false];

            for (index, expected) in expected.iter().enumerate() {
                assert_eq!(
                    is_safe(&TEST_DATA[index].to_vec(), 1),
                    *expected,
                    "testing iteration {}",
                    index
                );
            }
        }
    }
}
