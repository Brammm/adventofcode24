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
        println!("Total kinda valid reports is: {}", count_kinda_valid_reports(&reports));
    }

    fn count_valid_reports(reports: &Vec<Vec<i32>>) -> i32 {
        reports.iter().filter(|data| is_safe(data, 0)).count() as i32
    }

    fn count_kinda_valid_reports(reports: &Vec<Vec<i32>>) -> i32 {
        reports.iter().filter(|data| is_safe(data, 1)).count() as i32
    }

    fn is_safe(data: &Vec<i32>, tolerance: i32) -> bool {
        let pairs = data.len() - (1 + tolerance as usize);
        let decreasing_count = get_decreasing_pairs_count(data);
        let increasing_count = get_increasing_pairs_count(data);

        (decreasing_count >= pairs || increasing_count >= pairs) && is_within_limits(data, tolerance)
    }

    fn get_decreasing_pairs_count(data: &Vec<i32>) -> usize {
        data.windows(2).filter(|pair| pair[0] > pair[1]).count()
    }

    fn get_increasing_pairs_count(data: &Vec<i32>) -> usize {
        data.windows(2).filter(|pair| pair[0] < pair[1]).count()
    }

    fn is_within_limits(data: &Vec<i32>, tolerance: i32) -> bool {
        // piets eerste element van den array (als da faalt bv array niet lang genoeg, return false)
        if let Some((first, rest)) = data.split_first() {
            let mut previous = first;
            let mut error_count = 0;
    
            // loop over de rest van de items, vergelijkend met "previous", eerste loop dus eerste element
            for value in rest {
                let diff = (value - previous).abs(); // bereken absolute verschil
    
                if ![1, 2, 3].contains(&diff) { // difference is kleiner dan 1, groter dan 3
                    if error_count == tolerance { // als we aan onze tolerance zitten, false
                        return false;
                    } else {
                        error_count += 1; // nog niet aan onze tolerance, verhoog error count en vergelijk met volgend nummer
                        continue;
                    }
                }
    
                previous = value; // boel is nog geldig dus overwrite previous
            }
    
            return true;
        }
    
        false
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
        fn test_is_perfectly_safe() {
            let expected = vec![true, false, false, false, false, true];
        
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
            let expected = vec![true, false, false, true, true, true];

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
