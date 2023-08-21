//! # Integration tests
//!
//! Tests on the `extract` dataset

use itertools::Itertools;
use qrlew_sarus::data_spec::Dataset;
use qrlew::{ast::Query, display::Dot as _};
use std::fs::File;
use std::io::{self, Read};
use std::env;

fn read_file(filename: &str) -> String {
    let file_dir = env::current_dir().unwrap().join("tests").join("extract").join(filename);
    let mut file = File::open(file_dir).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content
}


#[test]
fn test_dataset() {
    // Open the file in read mode
    let dataset = read_file("dataset.txt");
    let schema = read_file("schema.txt");
    let size = read_file("size.txt");

    let dataset = Dataset::parse_from_dataset_schema_size(
        dataset.as_str(),
        schema.as_str(),
        size.as_str()
    ).expect("dataset");

    for (path, relation) in dataset.relations() {
        println!("{}", path.into_iter().join("."));
        relation.display_dot();
        println!("{}", Query::from(relation.as_ref()));
    }
}

