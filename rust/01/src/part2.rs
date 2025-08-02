use std::collections::HashMap;

fn split_columns(data: &str) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in data.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if nums.len() == 2 {
            col1.push(nums[0]);
            col2.push(nums[1]);
        }
    }

    // Sort from smallest to largest
    col1.sort();
    col2.sort();

    (col1, col2)
}

fn count_frequencies(numbers: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();

    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    counts
}

fn calculate_score(numbers: &[i32], frequency: HashMap<i32, usize>) -> i32 {
    let mut score = 0;

    for &num in numbers {
        let freq = *frequency.get(&num).unwrap_or(&0) as i32;
        score += num * freq;
    }

    score
}

fn solution(path: &str) -> i32 {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        eprintln!("File not found: {path}");
        return 0;
    }

    // Import the data
    let data = std::fs::read_to_string(path_obj).expect("Failed to read file");
    // Split the columns
    let (col1, col2) = split_columns(&data);
    // Create a hash map of frequencies
    let frequency = count_frequencies(&col2);
    // Calculat the score
    calculate_score(&col1, frequency)
}

pub fn print_solution(test_val: i32) {
    if solution("data/test2") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
