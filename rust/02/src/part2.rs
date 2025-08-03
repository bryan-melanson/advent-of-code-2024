fn row_to_vec(data: String) -> Vec<Vec<i32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn abs_diff_safe(a: i32, b: i32) -> bool {
    (1..=3).contains(&a.abs_diff(b))
}

fn is_strict_and_safe(row: &[i32]) -> bool {
    let mut direction = None;

    for i in 1..row.len() {
        if row[i] == row[i - 1] {
            return false;
        } else if direction.is_none() {
            direction = Some(row[i] > row[i - 1]);
        } else if direction.unwrap() != (row[i] > row[i - 1]) {
            return false;
        }

        if !abs_diff_safe(row[i], row[i - 1]) {
            return false;
        }
    }

    true
}

fn is_row_safe_with_removal(row: &[i32]) -> bool {
    if is_strict_and_safe(row) {
        return true;
    }

    // Remove an element
    for i in 0..row.len() {
        let mut copy = Vec::with_capacity(row.len() - 1);
        copy.extend_from_slice(&row[..i]);
        copy.extend_from_slice(&row[i + 1..]);
        if is_strict_and_safe(&copy) {
            return true;
        }
    }

    false
}

fn solution(path: &str) -> i32 {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        eprintln!("File not found: {path}");
        return 0;
    }

    // Import the data
    let data = std::fs::read_to_string(path_obj).expect("Failed to read file");
    // Split rows into Vecs
    let rows = row_to_vec(data);
    // Check each row for safety
    let count = rows
        .iter()
        .filter(|row| is_row_safe_with_removal(row))
        .count() as i32;

    count
}

pub fn print_solution(test_val: i32) {
    if solution("data/test2") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
