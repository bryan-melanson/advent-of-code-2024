use regex::Regex;

fn solution(path: &str) -> i32 {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        eprintln!("File not found: {path}");
        return 0;
    }

    // Regex to match mul(x, y) where x and y are signed integers
    let re = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();

    // Import the data
    let data = std::fs::read_to_string(path_obj).expect("Failed to read file");

    let mut total = 0;

    for cap in re.captures_iter(&data) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        total += x * y;
    }

    total
}

pub fn print_solution(test_val: i32) {
    if solution("data/test1") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
