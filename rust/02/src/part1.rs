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
    let diff = a.abs_diff(b);
    (1..=3).contains(&diff)
}

fn is_row_safe(row: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..row.len() {
        if !abs_diff_safe(row[i], row[i - 1]) {
            return false;
        }
        if row[i] >= row[i - 1] {
            decreasing = false;
        } else if row[i] <= row[i - 1] {
            increasing = false;
        } else {
            return false;
        }
    }

    increasing || decreasing
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
    let count = rows.iter().filter(|row| is_row_safe(row)).count() as i32;

    count
}

pub fn print_solution(test_val: i32) {
    if solution("data/test1") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
