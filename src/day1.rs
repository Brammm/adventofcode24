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
        
        left.sort();
        right.sort();
        
        let mut total_distance: i32 = 0;
        
        for (i, left_val) in left.iter().enumerate() {
            let right_val = right[i];
            total_distance = total_distance + (left_val - right_val).abs();
        }
        
        println!("Total distance is: {}", total_distance);
    }
}
