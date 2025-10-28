use std::collections::HashMap;

const DATA: &str = include_str!("vendor/detect-writing-script/data.json");

fn main() {
    let _: HashMap<String, Vec<[usize; 2]>> = serde_json::from_str(DATA).unwrap();
}
