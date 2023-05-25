//! Inspiration:
//! https://calcite.apache.org/docs/algebra.html
//! https://www.w3schools.com/sql/default.asp
//! https://www.sqlalchemy.org/
//! https://www.postgresql.org/docs/14/index.html

pub mod examples;

use std::{fmt, error, result, rc::Rc};
use std::str::FromStr;
use chrono::{self, NaiveDate, NaiveTime, NaiveDateTime, Duration};
use qrlew::{
    data_type::{self, DataType},
    expr::identifier::Identifier,
    relation::{Relation, Variant as _, schema::Schema},
    builder::{With, Ready},
    hierarchy::{Hierarchy, Path},
};
use crate::{
    protobuf::{dataset, schema, size, type_, statistics, print_to_string, parse_from_str, ParseError},
};

// Error management

#[derive(Debug, Clone)]
pub enum Error {
    ParsingError(String),
    Other(String),
}

impl Error {
    pub fn parsing_error(input: impl fmt::Display) -> Error { Error::ParsingError(format!("Cannot parse {}", input)) }
    pub fn other<T: fmt::Display>(desc: T) -> Error { Error::Other(desc.to_string()) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParsingError(input) => writeln!(f, "ParsingError: {}", input),
            Error::Other(err) => writeln!(f, "{}", err),
        }
    }
}

impl error::Error for Error {}

impl From<ParseError> for Error { fn from(err: ParseError) -> Self { Error::ParsingError(err.to_string()) } }
impl From<chrono::ParseError> for Error { fn from(err: chrono::ParseError) -> Self { Error::ParsingError(err.to_string()) } }

pub type Result<T> = result::Result<T, Error>;

/*
Definition of the dataset
 */

#[derive(Debug, Clone, PartialEq)]
pub struct Dataset {
    dataset: dataset::Dataset,
    schema: schema::Schema,
    size: Option<size::Size>,
}

impl Dataset {
    pub fn new( dataset: dataset::Dataset, schema: schema::Schema, size: Option<size::Size> ) -> Dataset {
        Dataset {
            dataset,
            schema,
            size,
        }
    }

    pub fn parse_from_dataset_schema_size(dataset: &str, schema: &str, size: &str) -> Result<Dataset> {
        Ok(Dataset::new(
            parse_from_str(dataset)?,
            parse_from_str(schema)?,
            parse_from_str(size).ok()
        ))
    }

    /// Return the data part of the schema
    fn schema_type_data(&self) -> &type_::Type {
        match self.schema.type_().type_.as_ref() {
            Some(type_::type_::Type::Struct(s)) => s.fields().iter().find_map(|f| if f.name()=="data" {
                    Some(f.type_())
                } else {
                    Some(self.schema.type_())
                }).unwrap(),
            _ => panic!("No data found in the type"),
        }
    }

    /// Return the data part of the schema
    pub fn size(&self) -> Option<& statistics::Statistics> {
        self.size.as_ref().map(|s| s.statistics())
    }

    pub fn relations(&self) -> Hierarchy<Rc<Relation>> {
        table_structs(self.schema_type_data(), self.size()).into_iter().map(|(i, t, s)|{
            let schema: Schema = t.try_into().unwrap();
            let mut builder = Relation::table().schema(schema);
            // Create a table builder with a name
            if let Some(name) = i.last() {
                builder = builder.name(name)
            }
            if let Some(s) = s {
                builder = builder.size(s.size())
            }
            (i, Rc::new(builder.build()))
        }).collect()
    }
}

/// Display a dataset
impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dataset: {}\nSchema: {}\nSize: {}",
            print_to_string(&self.dataset).unwrap(),
            print_to_string(&self.schema).unwrap(),
            self.size.as_ref().map_or(String::new(), |s| print_to_string(s).unwrap()))
    }
}

/// Parse a string into a dataset
impl FromStr for Dataset {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(";").into_iter().collect();
        if input.len()>=2 {
            Ok(Dataset::new(
                parse_from_str(input[0])?,
                parse_from_str(input[1])?,
                input.get(2).and_then(|&s| parse_from_str(s).ok())
            ))
        } else {
            Err(Error::parsing_error(s))
        }
    }
}

/*
A few utilities to visit types and statistics
 */
fn table_structs<'a>(t: &'a type_::Type, s: Option<&'a statistics::Statistics>) -> Vec<(Identifier, &'a type_::type_::Struct, Option<&'a statistics::statistics::Struct>)> {
    if let Some(t) = t.type_.as_ref() {
        match t {
            type_::type_::Type::Struct(t) => {// If the type is a Struct
                let s = s
                    .and_then(|s| s.statistics.as_ref())
                    .and_then(|s| match s {
                        statistics::statistics::Statistics::Struct(s) => Some(s),
                        _ => None,
                    }
                );
                vec![(Identifier::empty(), t, s)]
            },
            type_::type_::Type::Union(t) => {// If the type is a Union
                let s = s
                    .and_then(|s| s.statistics.as_ref())
                    .and_then(|s| match s {
                        statistics::statistics::Statistics::Union(s) => Some(s),
                        _ => None,
                    }
                );
                t.fields().iter().flat_map(|f| {
                    let g = s.and_then(|s| s.fields().iter().find_map(|g| if g.name()==f.name() {
                            Some(g.statistics())
                        } else {
                            None
                        })
                    );
                    table_structs(f.type_(), g).into_iter().map(|(i, t, s)| (i.with((0, f.name().to_string())), t, s))
                }).collect()
            },
            _ => Vec::new()
        }
    } else {
        Vec::new()
    }
}

/// Builds a DataType from a protobuf Type
impl<'a> From<&'a type_::Type> for DataType {
    fn from(value: &'a type_::Type) -> Self {
        value.type_.as_ref().map_or(DataType::Any, |t| match t {
            type_::type_::Type::Null(type_::type_::Null {special_fields}) => {
                DataType::Null
            },
            type_::type_::Type::Unit(type_::type_::Unit {special_fields}) => {
                DataType::unit()
            },
            type_::type_::Type::Boolean(type_::type_::Boolean {special_fields}) => {
                DataType::boolean()
            },
            type_::type_::Type::Integer(type_::type_::Integer { min, max, possible_values, ..}) => {
                if possible_values.len()>0 {
                    DataType::integer_values(possible_values)
                } else {
                    DataType::integer_interval(*min, *max)
                }
            },
            type_::type_::Type::Enum(type_::type_::Enum { name_values, ..}) => {
                DataType::Enum(name_values.iter().map(|nv|(nv.name(), nv.value())).collect())
            },
            type_::type_::Type::Float(type_::type_::Float { min, max, possible_values, ..}) => {
                if possible_values.len()>0 {
                    DataType::float_values(possible_values)
                } else {
                    DataType::float_interval(*min, *max)
                }
            },
            type_::type_::Type::Text(type_::type_::Text { possible_values, ..}) => {
                if possible_values.len()>0 {
                    DataType::text_values(possible_values)
                } else {
                    DataType::text()
                }
            },
            type_::type_::Type::Bytes(type_::type_::Bytes {special_fields}) => {
                DataType::bytes()
            },
            type_::type_::Type::Struct(type_::type_::Struct {fields, special_fields}) => {
                DataType::Struct(data_type::Struct::new(
                    fields.iter()
                        .map(|f|(f.name().to_string(), Rc::new(f.type_().into())))
                        .collect()
                ))
            },
            type_::type_::Type::Union(type_::type_::Union {fields, special_fields}) => {
                DataType::Union(data_type::Union::new(
                    fields.iter()
                        .map(|f|(f.name().to_string(), Rc::new(f.type_().into())))
                        .collect()
                ))
            },
            type_::type_::Type::Optional(type_::type_::Optional { type_, ..}) => {
                DataType::optional(type_.get_or_default().into())
            },
            type_::type_::Type::List(type_::type_::List {type_, max_size, special_fields}) => {
                DataType::list(type_.get_or_default().into(), 0, *max_size as usize)
            },
            type_::type_::Type::Array(type_::type_::Array {type_, shape, special_fields}) => {
                DataType::Array(data_type::Array::new(
                    Rc::new(type_.get_or_default().into()),
                    shape.iter().map(|x| *x as usize).collect()
                ))
            },
            type_::type_::Type::Date(type_::type_::Date {format, min, max, possible_values, base, special_fields}) => {
                if possible_values.len()>0 {
                    let possible_dates: Result<Vec<NaiveDate>> = possible_values.iter()
                        .map(|d| Ok(NaiveDate::parse_from_str(d, format)?))
                        .collect();
                    possible_dates.map_or_else(|e| DataType::Any, |possible_dates| DataType::date_values(possible_dates))
                } else {
                    NaiveDate::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveDate::parse_from_str(max, format)?)))
                        .map_or_else(|e| DataType::Any, |(min, max)| DataType::date_interval(min, max))
                }
            },
            type_::type_::Type::Time(type_::type_::Time {format, min, max, possible_values, base, special_fields}) => {
                if possible_values.len()>0 {
                    let possible_times: Result<Vec<NaiveTime>> = possible_values.iter()
                        .map(|d| Ok(NaiveTime::parse_from_str(d, format)?))
                        .collect();
                    possible_times.map_or_else(|e| DataType::Any, |possible_times| DataType::time_values(possible_times))
                } else {
                    NaiveTime::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveTime::parse_from_str(max, format)?)))
                        .map_or_else(|e| DataType::Any, |(min, max)| DataType::time_interval(min, max))
                }
            },
            type_::type_::Type::Datetime(type_::type_::Datetime {
                format,
                min,
                max,
                possible_values,
                base,
                special_fields}
            ) => {
                if possible_values.len()>0 {
                    let possible_date_times: Result<Vec<NaiveDateTime>> = possible_values.iter()
                        .map(|d| Ok(NaiveDateTime::parse_from_str(d, format)?))
                        .collect();
                    possible_date_times.map_or_else(|e| DataType::Any, |possible_date_times| DataType::date_time_values(possible_date_times))
                } else {
                    NaiveDateTime::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveDateTime::parse_from_str(max, format)?)))
                        .map_or_else(|e| DataType::Any, |(min, max)| DataType::date_time_interval(min, max))
                }
            },
            type_::type_::Type::Duration(type_::type_::Duration {unit, min, max, possible_values, special_fields}) => {
                let format_duration = match unit.as_str() {
                    "us" => Duration::microseconds,
                    "ms" => Duration::milliseconds,
                    "s" => Duration::seconds,
                    _ => panic!("stop"),
                };
                if possible_values.len()>0 {
                    let possible_date_times: Result<Vec<Duration>> = possible_values.iter()
                        .map(|d| Ok(format_duration(*d)))
                        .collect();
                    possible_date_times.map_or_else(|e| DataType::Any, |d| DataType::duration_values(d))
                } else {
                    DataType::duration_interval(format_duration(*min), format_duration(*max))
                }
            },
            type_::type_::Type::Id(type_::type_::Id { unique, reference, ..}) => {
                DataType::Id(data_type::Id::new(None, *unique))
            },
            _ => {
                DataType::Any
            },
        })
    }
}

/// Builds a Table Schema out of a Sarus Struct
impl<'a> From<&'a type_::type_::Struct> for Schema {
    fn from(t: &'a type_::type_::Struct) -> Self {
        t.fields().iter().map(|f| (f.name(), DataType::from(f.type_()))).collect()
    }
}

fn relation<'a>(i: Identifier, t: &'a type_::type_::Struct, s: Option<&'a statistics::statistics::Struct>) -> Relation {
    let schema: Schema = t.try_into().unwrap();
    let mut builder = Relation::table().schema(schema);
    // Create a table builder with a name
    if let Some(name) = i.last() {
        builder = builder.name(name)
    }
    if let Some(s) = s {
        builder = builder.size(s.size())
    }
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::str::FromStr;

    #[test]
    fn test_unit() -> Result<()> {
        let type_str: &str = r#"
           {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "name": "Unit",
            "properties": {},
            "unit": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::Unit(data_type::Unit));
        Ok(())
    }

    #[test]
    fn test_boolean() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "boolean": {},
                "name": "Boolean",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        //assert!(data_type == DataType::Boolean(data_type::Boolean::default()));
        Ok(())
    }

    #[test]
    fn test_integer() -> Result<()> {
        let type_str: &str = r#"
            {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "integer": {
            "base": "INT64",
            "max": "100",
            "min": "0",
            "possible_values": [
                "0",
                "1",
                "5",
                "99",
                "100"
            ]
            },
            "name": "Integer",
            "properties": {}
        }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::integer_values(vec![0, 1, 5, 99, 100]));

        let type_str: &str = r#"
            {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "integer": {
            "base": "INT64",
            "max": "100",
            "min": "0",
            "possible_values": []
            },
            "name": "Integer",
            "properties": {}
        }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::integer_interval(0, 100));
        Ok(())
    }

    #[test]
    fn test_enum() -> Result<()> {
        let type_str: &str = r#"
            {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "enum": {
              "base": "INT64",
              "name_values": [
                {
                  "name": "france",
                  "value": "0"
                },
                {
                  "name": "uk",
                  "value": "1"
                },
                {
                  "name": "usa",
                  "value": "2"
                }
              ],
              "ordered": true
            },
            "name": "country",
            "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let my_vec: Vec<(String, i64)> = vec![
            ("france".to_string(), 0),
            ("uk".to_string(), 1),
            ("usa".to_string(), 2),
        ];

        let my_rc_vec: Rc<[(String, i64)]> = Rc::from(my_vec);
        assert!(sarus_type == DataType::Enum(data_type::Enum::new(my_rc_vec)));
        println!("{:?}", sarus_type);
        Ok(())
    }

    #[test]
    fn test_float() -> Result<()> {
        let type_str: &str = r#"
            {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "float": {
              "base": "FLOAT64",
              "max": 1.0,
              "min": 0.0,
              "possible_values": [
                0.0,
                5.0,
                1.0
              ]
            },
            "name": "Float64",
            "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::float_values(vec![0., 5., 1.]);
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        let type_str: &str = r#"
            {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "float": {
              "base": "FLOAT64",
              "max": 1.0,
              "min": 0.0,
              "possible_values": []
            },
            "name": "Float64",
            "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::float_interval(0., 1.));
        Ok(())
    }

    #[test]
    fn test_text() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Text UTF-8",
                "properties": {},
                "text": {
                "encoding": "UTF-8",
                "possible_values": [
                    "a",
                    "b",
                    "c"
                ]
                }
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::text_values(vec!["a".to_string(), "b".to_string(), "c".to_string()]));

        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Text UTF-8",
                "properties": {},
                "text": {
                "encoding": "UTF-8",
                "possible_values": []
                }
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::text());
        Ok(())
    }

    #[test]
    fn test_bytes() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "bytes": {},
                "name": "Bytes",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::Bytes(data_type::Bytes));
        Ok(())
    }

    #[test]
    fn test_date() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "date": {
                "base": "INT32",
                "format": "%m/%d/%Y",
                "max": "01/01/2100",
                "min": "01/01/2000",
                "possible_values": []
                },
                "name": "Date",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::date_interval(
            NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d")?,
            NaiveDate::parse_from_str("2100-01-01", "%Y-%m-%d")?,
        );
        assert!(sarus_type == ok_results);


        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "date": {
                "base": "INT32",
                "format": "%m/%d/%Y",
                "max": "01/01/2100",
                "min": "01/01/2000",
                "possible_values": [
                    "01/01/2000",
                    "01/01/2001",
                    "01/01/2100"
                ]
                },
                "name": "Date",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::date_values(
            vec![
                NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d")?,
                NaiveDate::parse_from_str("2001-01-01", "%Y-%m-%d")?,
                NaiveDate::parse_from_str("2100-01-01", "%Y-%m-%d")?,
            ]
        );
        assert!(sarus_type == ok_results);
        Ok(())
    }

    #[test]
    fn test_time() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Time",
                "properties": {},
                "time": {
                "base": "INT64_US",
                "format": "%H:%M:%S.%f",
                "max": "12:12:03.000000",
                "min": "12:12:01.000000",
                "possible_values": []
                }
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::time_interval(
            NaiveTime::parse_from_str("12:12:01.000000", "%H:%M:%S.%f")?,
            NaiveTime::parse_from_str("12:12:03.000000", "%H:%M:%S.%f")?,
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);


        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Time",
                "properties": {},
                "time": {
                "base": "INT64_US",
                "format": "%H:%M:%S.%f",
                "max": "12:12:03.000000",
                "min": "12:12:01.000000",
                "possible_values": [
                    "12:12:01.000000",
                    "12:12:02.000000",
                    "12:12:03.000000"
                ]
                }
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::time_values(
            vec![
                NaiveTime::parse_from_str("12:12:01.000000", "%H:%M:%S.%f")?,
                NaiveTime::parse_from_str("12:12:02.000000", "%H:%M:%S.%f")?,
                NaiveTime::parse_from_str("12:12:03.000000", "%H:%M:%S.%f")?,
            ]
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        Ok(())
    }

    #[test]
    fn test_datetime() -> Result<()> {
        let type_str: &str = r#"
        {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "datetime": {
              "base": "INT64_NS",
              "format": "%Y-%m-%d %H:%M:%S",
              "max": "2023-12-31 00:00:00",
              "min": "2023-01-01 00:00:00",
              "possible_values": []
            },
            "name": "Datetime",
            "properties": {}
          }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::date_time_interval(
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?,
            NaiveDateTime::parse_from_str("2023-12-31 00:00:00", "%Y-%m-%d %H:%M:%S")?,
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);


        let type_str: &str = r#"
        {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "datetime": {
              "base": "INT64_NS",
              "format": "%Y-%m-%d %H:%M:%S",
              "max": "2023-12-31 00:00:00",
              "min": "2023-01-01 00:00:00",
              "possible_values": [
                "2023-01-01 00:10:00",
                "2023-06-01 00:00:30",
                "2023-12-01 11:00:00"
              ]
            },
            "name": "Datetime",
            "properties": {}
          }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::date_time_values(
            vec![
                NaiveDateTime::parse_from_str("2023-01-01 00:10:00", "%Y-%m-%d %H:%M:%S")?,
                NaiveDateTime::parse_from_str("2023-06-01 00:00:30", "%Y-%m-%d %H:%M:%S")?,
                NaiveDateTime::parse_from_str("2023-12-01 11:00:00", "%Y-%m-%d %H:%M:%S")?,
            ]
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_duration() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "duration": {
                "max": "3234567",
                "min": "1234567",
                "possible_values": [],
                "unit": "us"
                },
                "name": "Duration",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::duration_interval(
            Duration::microseconds(1234567),
            Duration::microseconds(3234567),
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "duration": {
                "max": "3234567",
                "min": "1234567",
                "possible_values": [
                    "1234567",
                    "2234567",
                    "3234567"
                ],
                "unit": "us"
                },
                "name": "Duration",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::duration_values(
            vec![
                Duration::microseconds(1234567),
                Duration::microseconds(2234567),
                Duration::microseconds(3234567),
            ]
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_id() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "id": {
                "base": "STRING",
                "unique": false
                },
                "name": "Id",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::id());
        Ok(())
    }

    #[test]
    fn test_struct() -> Result<()> {
        let type_str: &str = r#"
        {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "name": "Struct",
            "properties": {},
            "struct": {
              "fields": [
                {
                  "name": "integer_possible_values",
                  "type": {
                    "integer": {
                      "base": "INT64",
                      "max": "9223372036854775807",
                      "min": "-9223372036854775808",
                      "possible_values": []
                    },
                    "name": "Integer",
                    "properties": {}
                  }
                },
                {
                  "name": "text",
                  "type": {
                    "name": "Text UTF-8",
                    "properties": {},
                    "text": {
                      "encoding": "UTF-8",
                      "possible_values": [
                        "a",
                        "b",
                        "c"
                      ]
                    }
                  }
                }
              ]
            }
          }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::Struct(data_type::Struct::new(vec![
            ("integer_possible_values".to_string(), Rc::new(DataType::integer_interval(-9223372036854775808, 9223372036854775807))),
            ("text".to_string(), Rc::new(DataType::text_values(vec!["a".to_string(), "b".to_string(), "c".to_string()]))),
        ]));
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_union() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Union",
                "properties": {},
                "union": {
                "fields": [
                    {
                    "name": "integer_possible_values",
                    "type": {
                        "integer": {
                        "base": "INT64",
                        "max": "9223372036854775807",
                        "min": "-9223372036854775808",
                        "possible_values": []
                        },
                        "name": "Integer",
                        "properties": {}
                    }
                    },
                    {
                    "name": "text",
                    "type": {
                        "name": "Text UTF-8",
                        "properties": {},
                        "text": {
                        "encoding": "UTF-8",
                        "possible_values": [
                            "a",
                            "b",
                            "c"
                        ]
                        }
                    }
                    }
                ]
                }
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::Union(data_type::Union::new(vec![
            ("integer_possible_values".to_string(), Rc::new(DataType::integer_interval(-9223372036854775808, 9223372036854775807))),
            ("text".to_string(), Rc::new(DataType::text_values(vec!["a".to_string(), "b".to_string(), "c".to_string()]))),
        ]));
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_optional() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "name": "Optional",
                "optional": {
                "type": {
                    "integer": {
                    "base": "INT64",
                    "max": "9223372036854775807",
                    "min": "-9223372036854775808",
                    "possible_values": []
                    },
                    "name": "Integer",
                    "properties": {}
                }
                },
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::optional(
            DataType::integer_interval(-9223372036854775808, 9223372036854775807)
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_list() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "list": {
                "max_size": "5",
                "type": {
                    "integer": {
                    "base": "INT64",
                    "max": "9223372036854775807",
                    "min": "-9223372036854775808",
                    "possible_values": []
                    },
                    "name": "Integer",
                    "properties": {}
                }
                },
                "name": "List",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        let ok_results = DataType::list(
            DataType::integer_interval(-9223372036854775808, 9223372036854775807), 0, 5
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);

        Ok(())
    }

    #[test]
    fn test_set() -> Result<()> {
        // Not implemented in data spec
        Ok(())
    }

    #[test]
    fn test_array() -> Result<()> {
        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "array": {
                "shape": [
                    "1"
                ],
                "type": {
                    "integer": {
                    "base": "INT64",
                    "max": "9223372036854775807",
                    "min": "-9223372036854775808",
                    "possible_values": []
                    },
                    "name": "Integer",
                    "properties": {}
                }
                },
                "name": "Array",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        println!("{:?}", sarus_type);
        let ok_results = DataType::Array(data_type::Array::new(
            Rc::new(DataType::integer_interval(-9223372036854775808, 9223372036854775807)),
            Rc::new([1])
        ));
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);



        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_retail_demo() -> Result<()> {
        let dataset = Dataset::parse_from_dataset_schema_size(
            examples::retail_demo::DATASET,
            examples::retail_demo::SCHEMA,
            examples::retail_demo::SIZE,
        )?;

        println!("{}", dataset.relations());
        for (i, r) in dataset.relations() {
            println!("{}", r);
            println!(" size = {}", r.size());
            println!(" schema = {}", r.schema());
            println!("---");
        }
        Ok(())
    }
}
