pub mod day1 {
    use std::fs;

    pub fn run() {
        let contents = fs::read_to_string("input/day1").expect("File not found");

        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in contents.lines() {
            if let Some((first, second)) = line.split_once(char::is_whitespace) {
                // Parse and push into respective vectors
                left.push(first.trim().parse::<i32>().expect("Invalid left number"));
                right.push(second.trim().parse::<i32>().expect("Invalid right number"));
            }
        }

        println!("Total distance is: {}", total_distance(&left, &right));
        println!("Similarity score is: {}", similarity_score(&left, &right));
    }

    fn total_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
        let mut left = left.to_vec();
        let mut right = right.to_vec();

        left.sort();
        right.sort();

        let mut total_distance: i32 = 0;

        for (i, left_val) in left.iter().enumerate() {
            let right_val = right[i];
            total_distance = total_distance + (left_val - right_val).abs();
        }

        total_distance
    }

    fn similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
        let mut similarity_score: i32 = 0;

        for left_val in left {
            let occurrence = right.iter().filter(|&&x| x == *left_val).count() as i32;
            similarity_score = similarity_score + left_val * occurrence;
        }

        similarity_score
    }
}
