use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DATA: &str = include_str!("vendor/detect-writing-script/data.json");

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

fn main() {
    let data: BTreeMap<String, Vec<[u32; 2]>> = ok!(serde_json::from_str(DATA));
    let script_count = data.len();

    let root = ok!(std::env::var("OUT_DIR"));
    let path = Path::new(&root).join("data.rs");
    let mut file = ok!(File::create(&path));
    ok!(write!(
        file,
        r#"
const SCRIPT_COUNT: usize = {script_count};
        "#,
    ));
}
