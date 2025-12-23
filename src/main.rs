mod day6;
use std::fs;
use day6::day6;

#[derive(Debug, Copy, Clone)]
struct Range { start: i64, end: i64}

impl Range {
    fn contains(  &self, ingredient_id: i64 ) -> bool {
        ingredient_id >= self.start && ingredient_id <= self.end
    }
}

fn main() {
    // assignment1();
    // assignment2();

    day6();
}

fn assignment1() {
    let contents = fs::read_to_string("src/day5")
        .expect("Failed to read file");
    let mut ranges: Vec<Range> = Vec::new();

    let mut finding_ranges = true;

    let mut number_of_fresh_ingredients: i64 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            finding_ranges = false; //switch to checking ingredients
            continue;
        }

        if finding_ranges {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: i64 = start_str.parse().unwrap();
            let end: i64 = end_str.parse().unwrap();

            ranges.push(Range { start: start, end: end })
        } else {
            let ingredient_id: i64 = line.parse().unwrap();
            let is_fresh = ranges.iter().any(|range| range.contains(ingredient_id));
            if is_fresh {
                number_of_fresh_ingredients += 1;
            }
        }
    }

    println!("assignment 1");
    println!("{:#?}", ranges);
    println!("{}", number_of_fresh_ingredients);
}

fn assignment2() {
    let contents = fs::read_to_string("src/day5")
        .expect("Failed to read file");
    let mut ranges: Vec<Range> = Vec::new();
    let mut merged: Vec<Range> = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        let (start_str, end_str) = line.split_once('-').unwrap();
        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();

        ranges.push(Range { start: start, end: end })
    }

    ranges.sort_by_key(|range| range.start); //sort ranges on start
    merged.push(ranges[0]); // Start with first range

    for (range) in ranges.into_iter().skip(1) {
        let last_range_in_set = merged.last_mut().unwrap();

        if range.start <= last_range_in_set.end + 1 {
            last_range_in_set.end = last_range_in_set.end.max(range.end);  // Merge into last range
        } else {
            merged.push(range);  // Add as new range
        }
    }

    println!("assignment 2");
    let total_count: i64 = merged.iter()
        .map(|range| range.end - range.start + 1)
        .sum();

    println!("Total numbers covered: {}", total_count);
}