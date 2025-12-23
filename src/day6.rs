use std::fs;

pub fn day6 () {
    let file_contents = fs::read_to_string("src/day6").expect("Failed to read file");
    let contents: Vec<&str> = file_contents.lines().collect();

    let line1: Vec<i64> = split_digit_line(contents[0].to_string());
    let line2: Vec<i64> = split_digit_line(contents[1].to_string());
    let line3: Vec<i64> = split_digit_line(contents[2].to_string());
    let line4: Vec<i64> = split_digit_line(contents[3].to_string());

    let mut grand_total: i64 = 0;

    let operators: Vec<&str> = contents[4].split_whitespace().collect();

    for (i, &operator) in operators.iter().enumerate() {
        let a = line1[i];
        let b = line2[i];
        let c = line3[i];
        let d = line4[i];

        match operator {
            "+" => grand_total += (a + b + c + d),
            "*" => grand_total += (a * b * c * d),
            _ => panic!("Unknown operator"),
        };
    }

    println!("Grand total{}", grand_total);
}


fn split_digit_line(line: String) -> Vec<i64> {
    return line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}