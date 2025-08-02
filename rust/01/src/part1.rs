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

    // Sort from smalles to largest
    col1.sort();
    col2.sort();

    (col1, col2)
}

fn calculate_diffs(col1: Vec<i32>, col2: Vec<i32>) -> i32 {
    let mut total_diff = 0;

    for (val1, val2) in col1.iter().zip(col2.iter()) {
        if val1 < val2 {
            total_diff += val2 - val1;
        } else if val1 > val2 {
            total_diff += val1 - val2;
        }
    }
    total_diff
}

fn solution(path: &str) -> i32 {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        eprintln!("File not found: {path}");
        return 0;
    }

    // Import the data
    let data = std::fs::read_to_string(path_obj).expect("Failed to read file");
    // Split the columns and sort
    let (col1, col2) = split_columns(&data);
    // Calculte the difference
    calculate_diffs(col1, col2)
}

pub fn print_solution(test_val: i32) {
    if solution("data/test1") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
