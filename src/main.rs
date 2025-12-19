use std::fs;

#[derive(Debug)]
struct Range { start: i64, end: i64}

impl Range {
    fn contains(  &self, ingredient_id: i64 ) -> bool {
        ingredient_id >= self.start && ingredient_id <= self.end
    }
}

fn main() {
    let contents = fs::read_to_string("src/day5")
        .expect("Failed to read file");
    let mut ranges: Vec<Range> = Vec::new();

    let mut finding_ranges = true;

    let mut number_of_fresh_ingredients: i64 = 0;

    'lineIterator: for line in contents.lines() {

        if line.is_empty() {
            finding_ranges = false; //switch to checking ingredients
            continue;
        }

        if finding_ranges {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: i64 = start_str.parse().unwrap();
            let end: i64 = end_str.parse().unwrap();

            ranges.push(Range { start: start, end: end})
        } else {
            let ingredient_id: i64 = line.parse().unwrap();

            for range in &ranges {
                if range.contains(ingredient_id) {
                    number_of_fresh_ingredients += 1;
                    continue 'lineIterator;
                }
            }
        }
    }

    println!("{:#?}", ranges);
    println!("{}", number_of_fresh_ingredients);
}
