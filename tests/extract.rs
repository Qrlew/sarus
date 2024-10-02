//! # Integration tests
//!
//! Tests on the `extract` dataset

use itertools::Itertools;
use qrlew_sarus::data_spec::Dataset;
use qrlew::{ast::Query, display::Dot as _};

const DATASET: &str = r#"{"@type": "sarus_data_spec/sarus_data_spec.Dataset", "uuid": "e9cb9391ca184e89897f49bd75387a46", "name": "Transformed", "spec": {"transformed": {"transform": "98f18c2b0beb406088193dab26e24552", "arguments": [], "named_arguments": {}}}, "properties": {}, "doc": "This ia a demo dataset for testing purpose"}"#;
const SCHEMA: &str = r#"{"@type": "sarus_data_spec/sarus_data_spec.Schema", "uuid": "5321f24ffb324a9e958c77ceb09b6cc8", "dataset": "c0d13d2c5d404e2c9930e01f63e18cee", "name": "extract", "type": {"name": "extract", "struct": {"fields": [{"name": "sarus_data", "type": {"name": "Union", "union": {"fields": [{"name": "extract", "type": {"name": "Union", "union": {"fields": [{"name": "beacon", "type": {"name": "Struct", "struct": {"fields": [{"name": "\u691c\u77e5\u65e5\u6642", "type": {"name": "Datetime", "datetime": {"format": "%Y-%m-%d %H:%M:%S", "min": "01-01-01 00:00:00", "max": "9999-12-31 00:00:00"}, "properties": {}}}, {"name": "UserId", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "\u6240\u5c5e\u90e8\u7f72", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "\u30d5\u30ed\u30a2\u540d", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "Beacon\u540d", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "RSSI", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "\u30de\u30c3\u30d7\u306eX\u5ea7\u6a19", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "\u30de\u30c3\u30d7\u306eY\u5ea7\u6a19", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}]}, "properties": {}}}, {"name": "census", "type": {"name": "Struct", "struct": {"fields": [{"name": "age", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "workclass", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "fnlwgt", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "education", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "education_num", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "marital_status", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "occupation", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "relationship", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "race", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "sex", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "capital_gain", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "capital_loss", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "hours_per_week", "type": {"name": "Integer", "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "native_country", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "income", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}, "properties": {}}}]}, "properties": {}}}]}, "properties": {"public_fields": "[]"}}}]}, "properties": {"public_fields": "[]"}}}, {"name": "sarus_weights", "type": {"name": "Integer", "integer": {"min": "-9223372036854775808", "max": "9223372036854775807", "base": "INT64", "possible_values": []}, "properties": {}}}, {"name": "sarus_is_public", "type": {"name": "Boolean", "boolean": {}, "properties": {}}}, {"name": "sarus_protected_entity", "type": {"name": "Id", "id": {"base": "STRING", "unique": false}, "properties": {}}}]}, "properties": {}}, "protected": {"label": "data", "paths": [], "properties": {}}, "properties": {"max_max_multiplicity": "1", "foreign_keys": "", "primary_keys": ""}}"#;
const SIZE: &str = r#"{"@type": "sarus_data_spec/sarus_data_spec.Size", "uuid": "cd8ec3f7958e4b2c842bc66ffa55e40c", "dataset": "c0d13d2c5d404e2c9930e01f63e18cee", "name": "extract_sizes", "statistics": {"name": "Union", "union": {"fields": [{"name": "extract", "statistics": {"name": "Union", "union": {"fields": [{"name": "beacon", "statistics": {"name": "Struct", "size": "100", "multiplicity": 1.0, "struct": {"fields": [{"name": "\u691c\u77e5\u65e5\u6642", "statistics": {"name": "Datetime", "size": "100", "multiplicity": 1.0, "datetime": {"format": "%Y-%m-%d %H:%M:%S", "min": "01-01-01 00:00:00", "max": "9999-12-31 00:00:00"}, "properties": {}}}, {"name": "UserId", "statistics": {"name": "Text UTF-8", "size": "100", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "\u6240\u5c5e\u90e8\u7f72", "statistics": {"name": "Text UTF-8", "size": "100", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "\u30d5\u30ed\u30a2\u540d", "statistics": {"name": "Text UTF-8", "size": "100", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "Beacon\u540d", "statistics": {"name": "Text UTF-8", "size": "100", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "RSSI", "statistics": {"name": "Integer", "size": "100", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "\u30de\u30c3\u30d7\u306eX\u5ea7\u6a19", "statistics": {"name": "Integer", "size": "100", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "\u30de\u30c3\u30d7\u306eY\u5ea7\u6a19", "statistics": {"name": "Integer", "size": "100", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}]}, "properties": {}}}, {"name": "census", "statistics": {"name": "Struct", "size": "199", "multiplicity": 1.0, "struct": {"fields": [{"name": "age", "statistics": {"name": "Integer", "size": "199", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "workclass", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "fnlwgt", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "education", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "education_num", "statistics": {"name": "Integer", "size": "199", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "marital_status", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "occupation", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "relationship", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "race", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "sex", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "capital_gain", "statistics": {"name": "Integer", "size": "199", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "capital_loss", "statistics": {"name": "Integer", "size": "199", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "hours_per_week", "statistics": {"name": "Integer", "size": "199", "multiplicity": 1.0, "integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807", "possible_values": []}, "properties": {}}}, {"name": "native_country", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}, {"name": "income", "statistics": {"name": "Text UTF-8", "size": "199", "multiplicity": 1.0, "text": {"encoding": "UTF-8"}, "properties": {}}}]}, "properties": {}}}]}, "properties": {}}, "properties": {}}]}, "properties": {}}, "properties": {}}"#;

#[test]
fn test_dataset() {
    let dataset = Dataset::parse_from_dataset_schema_size(DATASET, SCHEMA, SIZE).expect("dataset");

    for (path, relation) in dataset.relations() {
        println!("{}", path.into_iter().join("."));
        relation.display_dot().unwrap();
        println!("{}", Query::from(relation.as_ref()));
    }
}

#[test]
fn test_dataset_bis() {
    let ds: &str = r#"{"@type": "sarus_data_spec/sarus_data_spec.Dataset", "uuid": "e9cb9391ca184e89897f49bd75387a46", "name": "Transformed", "spec": {"transformed": {"transform": "98f18c2b0beb406088193dab26e24552", "arguments": [], "named_arguments": {}}}, "properties": {}, "doc": "This ia a demo dataset for testing purpose"}"#;
    let sch: &str = r#"
    {
        "@type": "sarus_data_spec/sarus_data_spec.Schema",
        "uuid": "ee38558076e845ccb9c4baf7e9f8ba4a",
        "dataset": "b65e7defe8cb4da18741dec7192a8bc7",
        "name": "test", 
        "type": {
            "name": "test",
            "struct": {
                "fields": [{
                    "name": "user",
                    "type": {
                        "name": "Struct",
                        "struct": {
                            "fields": [
                                {"name": "user_id", "type": {"name": "Integer", "integer": {"min": "-9223372036854775808", "max": "9223372036854775807"}}},
                                {"name": "first_name", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}}},
                                {"name": "last_name", "type": {"name": "Text UTF-8", "text": {"encoding": "UTF-8"}}}
                            ]
                        }
                    }
                }]
            }
        }
    }
    "#;
    let size: &str = r#"{}"#;

    let dataset = Dataset::parse_from_dataset_schema_size(ds, sch, size).expect("dataset");
    println!("SCHEMA TYPE:\n {}\n", dataset.schema().type_());
    println!("HAS ADMIN COLS:\n {}\n", dataset.schema_has_admin_columns());
    println!("ADMIN NAMES:\n {:?}\n", dataset.admin_names_and_types());
    println!("SCHEMA DATA TYPE:\n {}\n", dataset.schema_type_data());


    for (path, relation) in dataset.relations() {
        println!("{}", path.into_iter().join("."));
        relation.display_dot().unwrap();
        println!("{}", Query::from(relation.as_ref()));
    }
}