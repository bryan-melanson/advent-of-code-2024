use regex::Regex;

fn solution(path: &str) -> i32 {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        eprintln!("File not found: {path}");
        return 0;
    }

    // Search for don't() <dont> do() <do> or mul(x,y) <x> <y>
    let re = Regex::new(
        r"(?x)
        (?P<do>do\(\)) |
        (?P<dont>don't\(\)) |
        mul\((?P<x>\d+),(?P<y>\d+)\)
    ",
    )
    .unwrap();

    let mut mult = true;

    // Import the data
    let data = std::fs::read_to_string(path_obj).expect("Failed to read file");

    let mut total = 0;

    for cap in re.captures_iter(&data) {
        if cap.name("do").is_some() {
            // If regex do group enabled mult
            mult = true;
        } else if cap.name("dont").is_some() {
            // If regex don't group disable mult
            mult = false;
        } else if let (Some(x), Some(y)) = (cap.name("x"), cap.name("y")) {
            // If regex x or y group multiply and add
            let x: i32 = x.as_str().parse().unwrap();
            let y: i32 = y.as_str().parse().unwrap();
            if mult {
                total += x * y;
            }
        }
    }

    total
}

pub fn print_solution(test_val: i32) {
    if solution("data/test2") != test_val {
        println!("Test failed");
    } else {
        println!("{}", solution("data/input"));
    }
}
