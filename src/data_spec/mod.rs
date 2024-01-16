//! Inspiration:
//! https://calcite.apache.org/docs/algebra.html
//! https://www.w3schools.com/sql/default.asp
//! https://www.sqlalchemy.org/
//! https://www.postgresql.org/docs/14/index.html

use crate::protobuf::{
    dataset, parse_from_str, print_to_string, schema, size, statistics, type_, ParseError, constraint,
};
use chrono::{self, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use qrlew::{
    builder::{Ready, With},
    data_type::{self, DataType},
    expr::identifier::Identifier,
    hierarchy::Hierarchy,
    relation::{schema::Schema, Constraint, Relation, Variant as _},
    data_type::DataTyped,
};
use std::{str::FromStr, error, fmt, sync::Arc, result, convert::{TryFrom, TryInto}, collections::HashSet};

pub const CONSTRAINT: &str = "_CONSTRAINT_";
pub const CONSTRAINT_UNIQUE: &str = "_UNIQUE_";// We ignore other constraints

// Error management

#[derive(Debug, Clone)]
pub enum Error {
    ParsingError(String),
    MissingKeyError(String),
    Other(String),
}

impl Error {
    pub fn parsing_error(input: impl fmt::Display) -> Error {
        Error::ParsingError(format!("Cannot parse {}", input))
    }
    pub fn missing_key_error(key: impl fmt::Display) -> Error {
        Error::MissingKeyError(format!("Cannot find key {}", key))
    }
    pub fn other<T: fmt::Display>(desc: T) -> Error {
        Error::Other(desc.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParsingError(input) => writeln!(f, "ParsingError: {}", input),
            Error::MissingKeyError(key) => writeln!(f, "MissingKeyError: {}", key),
            Error::Other(err) => writeln!(f, "{}", err),
        }
    }
}

impl error::Error for Error {}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::parsing_error(err)
    }
}
impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::parsing_error(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

/*
Definition of the dataset
 */

const SARUS_DATA: &str = "sarus_data";
const PID_COLUMN: &str = "sarus_protected_entity";
const WEIGHTS: &str = "sarus_weights";
const PUBLIC: &str = "sarus_is_public";


#[derive(Debug, Clone, PartialEq)]
pub struct Dataset {
    dataset: dataset::Dataset,
    schema: schema::Schema,
    size: Option<size::Size>,
}

impl Dataset {
    pub fn new(
        dataset: dataset::Dataset,
        schema: schema::Schema,
        size: Option<size::Size>,
    ) -> Dataset {
        Dataset {
            dataset,
            schema,
            size,
        }
    }

    // getters
    pub fn dataset(&self) -> &dataset::Dataset {
        &self.dataset
    }

    pub fn schema(&self) -> &schema::Schema {
        &self.schema
    }

    pub fn size(&self) -> Option<&size::Size> {
        self.size.as_ref()
    }

    pub fn size_statistics(&self) -> Option<&statistics::Statistics> {
        self.size.as_ref().map(|s| s.statistics())
    }

    pub fn parse_from_dataset_schema_size(
        dataset: &str,
        schema: &str,
        size: &str,
    ) -> Result<Dataset> {
        Ok(Dataset::new(
            parse_from_str(dataset)?,
            parse_from_str(schema)?,
            parse_from_str(size).ok(),
        ))
    }

    /// Returns the schema type
    pub fn schema_type(&self) -> &type_::Type {
        self.schema.type_()
    }

    /// Returns the SARUS_DATA type part of the schema type
    pub fn schema_type_data(&self) -> &type_::Type {
        match self.schema.type_().type_.as_ref() {
            Some(type_::type_::Type::Struct(s)) => {
                if let Some(data_type) = s
                    .fields()
                    .iter()
                    .find_map(|f| {
                        if f.name() == SARUS_DATA {
                            Some(f.type_())
                        } else { None }
                    }) {
                    data_type
                } else { self.schema_type() }
            }
            _ => self.schema_type()
        }
    }

    pub fn relations(&self) -> Hierarchy<Arc<Relation>> {
        let relations_without_prefix: Hierarchy<Arc<Relation>> = table_structs(self.schema_type_data(), self.size_statistics())
            .into_iter()
            .map(|(identifier, schema_struct, size_struct)| {
                (identifier.clone(), Arc::new(relation_from_struct(identifier, schema_struct, size_struct)))
            })
            .collect();
        let schema_name = self.schema().name();
        relations_without_prefix.prepend(&[schema_name.to_string()])
    }

    pub fn with_range(&self, schema_name: &str, table_name: &str, field_name: &str, min: f64, max: f64) -> Result<Self> {
        let mut new_schema = self.schema.clone();
        let mut sarus_data = new_schema.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==SARUS_DATA).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut schema = sarus_data.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==schema_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut table = schema.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==table_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut field = table.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==field_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        match &mut field.mut_type().type_ {
            Some(type_::type_::Type::Integer(integer)) => {
                integer.set_min(min.round() as i64);
                integer.set_max(max.round() as i64);
            },
            Some(type_::type_::Type::Float(float)) => {
                float.set_min(min);
                float.set_max(max);
            },
            Some(type_::type_::Type::Optional(optional)) => {
                match &mut optional.mut_type().type_ {
                    Some(type_::type_::Type::Integer(integer)) => {
                        integer.set_min(min.round() as i64);
                        integer.set_max(max.round() as i64);
                    },
                    Some(type_::type_::Type::Float(float)) => {
                        float.set_min(min);
                        float.set_max(max);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        Ok(Dataset::new(self.dataset.clone(), new_schema, self.size.clone()))
    }

    pub fn with_possible_values(&self, schema_name: &str, table_name: &str, field_name: &str, possible_values: &[String]) -> Result<Self> {
        let mut new_schema = self.schema.clone();
        let mut sarus_data = new_schema.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==SARUS_DATA).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut schema = sarus_data.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==schema_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut table = schema.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==table_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut field = table.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==field_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        match &mut field.mut_type().type_ {
            Some(type_::type_::Type::Text(text)) => {
                text.set_possible_values(possible_values.iter().cloned().collect());
            },
            Some(type_::type_::Type::Optional(optional)) => {
                match &mut optional.mut_type().type_ {
                    Some(type_::type_::Type::Text(text)) => {
                        text.set_possible_values(possible_values.iter().cloned().collect());
                    },
                    _ => (),
                }
            },
            _ => (),
        }
        Ok(Dataset::new(self.dataset.clone(), new_schema, self.size.clone()))
    }

    pub fn with_constraint(&self, schema_name: &str, table_name: &str, field_name: &str, constraint: Option<&str>) -> Result<Self> {
        let mut new_schema = self.schema.clone();
        let mut sarus_data = new_schema.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==SARUS_DATA).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut schema = sarus_data.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==schema_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut table = schema.mut_type().mut_union().mut_fields().iter_mut().find(|field| field.name()==table_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        let mut field = table.mut_type().mut_struct().mut_fields().iter_mut().find(|field| field.name()==field_name).ok_or_else(|| Error::missing_key_error(SARUS_DATA))?;
        if let Some(constraint) = constraint {
            field.mut_type().set_properties([(CONSTRAINT.to_string(), constraint.to_string())].into());
        } else {
            field.mut_type().mut_properties().remove(CONSTRAINT);
        }
        Ok(Dataset::new(self.dataset.clone(), new_schema, self.size.clone()))
    }

}

/// Display a dataset
impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Dataset: {}\nSchema: {}\nSize: {}",
            print_to_string(&self.dataset).unwrap(),
            print_to_string(&self.schema).unwrap(),
            self.size
                .as_ref()
                .map_or(String::new(), |s| print_to_string(s).unwrap())
        )
    }
}

/// Parse a string into a dataset
impl FromStr for Dataset {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(";").into_iter().collect();
        if input.len() >= 2 {
            Ok(Dataset::new(
                parse_from_str(input[0])?,
                parse_from_str(input[1])?,
                input.get(2).and_then(|&s| parse_from_str(s).ok()),
            ))
        } else {
            Err(Error::parsing_error(s))
        }
    }
}

/// Create a dataset from Relations
impl <'a> TryFrom<&'a Hierarchy<Arc<Relation>>> for Dataset {
    type Error = Error; 

    fn try_from(relations: &Hierarchy<Arc<Relation>>) -> Result<Self> {
        let dataset = dataset::Dataset::new();
        let path_prefixes_set = extract_paths_with_prefix(relations, &vec![]);
        if path_prefixes_set.len() > 1 {
            return Err(Error::Other("Relations have paths with not a unique head. Could not transform Relations into multiple Datasets.".to_string()))
        }
        let schema: schema::Schema = relations.try_into()?;
        let schema_name_path = vec![schema.name().to_string()];
        let size = match statistics_from_relations(relations, &schema_name_path) {
            Some(size_statistics) => {
                let mut size_proto = size::Size::new();
                size_proto.set_statistics(size_statistics);
                Some(size_proto)
            },
            None => None,
        };
        Ok(Dataset { dataset, schema, size })
    }
}

/// Try to build a Schema protobuf from relations
impl <'a> TryFrom<&'a Hierarchy<Arc<Relation>>> for schema::Schema {
    type Error = Error; 

    fn try_from(relations: &Hierarchy<Arc<Relation>>) ->  Result<Self> {
        let mut schema = schema::Schema::new();
        
        let common_paths: HashSet<Vec<String>> = extract_paths_with_prefix(relations, &vec![]);
        let schema_name_path= common_paths
        .iter()
        .next()
        .ok_or(
            Error::Other("Could not transform Relations with empty Path into Schema.".to_string())
        )?;
        schema.set_name(schema_name_path[0].clone());
        
        let mut schema_type = type_::Type::new();
        let mut first_level_struct = type_::type_::Struct::new();
        let mut first_level_proto_fields: Vec<type_::type_::struct_::Field> = vec![];

        let data_type = type_from_relations(relations,schema_name_path)?;
        let first_level_fields: Vec<(String, type_::Type)> = vec![
            (SARUS_DATA.to_string(), data_type),
            (PID_COLUMN.to_string(), (&DataType::optional(DataType::id())).try_into()?),
            (PUBLIC.to_string(), (&DataType::boolean()).try_into()?),
            (WEIGHTS.to_string(), (&DataType::float_interval(0.0 as f64, f64::MAX)).try_into()?),
        ];
        for (name, dtype) in first_level_fields.into_iter() {
            let mut data_field = type_::type_::struct_::Field::new();
            data_field.set_name(name.to_string());
            data_field.set_type(dtype);
            first_level_proto_fields.push(data_field)
        }
        first_level_struct.set_fields(first_level_proto_fields);

        schema_type.set_name("Struct".to_string());
        schema_type.set_struct(first_level_struct);
        schema.set_type(schema_type);
        Ok(schema)
    }
}

// Utility fuunctions

fn is_prefix_of(left: &[String], right: &[String]) -> bool {
    left.iter().zip(right.iter()).all(|(pr, pa)| pr == pa)
}

/// Returns a HashSet containing vectors of strings. Each vector represents
/// a unique path in the hierarchy that has the specified prefix.
fn extract_paths_with_prefix(relations: &Hierarchy<Arc<Relation>>, prefix: &Vec<String>) -> HashSet<Vec<String>>{
    relations
    .iter()
    .fold(HashSet::new(), |mut set, (path, _)|{
        if let Some(path_element) = path.get(prefix.len()) {
            if is_prefix_of(prefix, path) {
                set.insert(prefix.into_iter().chain([path_element].into_iter()).cloned().collect());
            }
        }
        set
    } )
}

/// Create a Type protobuf from relations
fn type_from_relations(relations: &Hierarchy<Arc<Relation>>, prefix: &Vec<String>) -> Result<type_::Type> {
    let common_paths: HashSet<Vec<String>> = extract_paths_with_prefix(relations, prefix);
    if common_paths.is_empty() {
        if let Some(rel) = relations.get(prefix) {
            let mut proto_struct = type_::type_::Struct::new();
            // TODO this is relatively ugly, to be changed
            for field in rel.schema().fields() {
                let mut proto_field = type_::type_::struct_::Field::new();
                let mut proto_field_type: type_::Type = (&field.data_type()).try_into()?;
                proto_field.set_name(field.name().to_string());
                if let Some(Constraint::Unique) = field.constraint() {
                    proto_field_type.set_properties([(CONSTRAINT.to_string(), CONSTRAINT_UNIQUE.to_string())].into());
                }
                proto_field.set_type(proto_field_type);
                proto_struct.fields.push(proto_field);
            }
            let mut proto_type = type_::Type::new();
            proto_type.set_name("Struct".into());
            proto_type.set_struct(proto_struct);
            Ok(proto_type)
        } else {
            return Err(Error::Other(
                "Could not convert relations into data type".to_string(),
            ))
        }
    } else {
        // create Unions
        let mut proto_type = type_::Type::new();
        let mut union_type = type_::type_::Union::new();
        let mut proto_fields: Vec<type_::type_::union::Field> = vec![];

        for path in common_paths.iter() {
            let field_name = &path[path.len()-1];
            let mut data_field = type_::type_::union::Field::new();
            data_field.set_name(field_name.clone());
            let dtype = type_from_relations(relations, path)?;
            data_field.set_type(dtype);
            proto_fields.push(data_field)
        }

        union_type.set_fields(proto_fields);
        proto_type.set_name("Union".to_string());
        proto_type.set_union(union_type);
        Ok(proto_type)
    }
}

/// Create a Statistics protobuf from relations
fn statistics_from_relations(relations: &Hierarchy<Arc<Relation>>, prefix: &Vec<String>) -> Option<statistics::Statistics> {
    let mut stat_proto = statistics::Statistics::new();
    let common_paths: HashSet<Vec<String>> = extract_paths_with_prefix(relations, prefix);
    if common_paths.is_empty() {
        if let Some(rel) = relations.get(prefix) {
            let mut struct_proto = statistics::statistics::Struct::new();
            if let Some(rel_size) = rel.size().max() {
                struct_proto.set_size(*rel_size);
                stat_proto.set_struct(struct_proto)
            };
        } else {
            return None
        };
    } else {
        let mut union_proto = statistics::statistics::Union::new();
        let mut proto_fields: Vec<statistics::statistics::union::Field> = vec![];
        for path in common_paths.iter() {
            if let Some(field_statistics) = statistics_from_relations(relations, path) {
                let field_name = &path[path.len()-1];
                let mut data_field = statistics::statistics::union::Field::new();
                data_field.set_name(field_name.clone());
                data_field.set_statistics(field_statistics);
                proto_fields.push(data_field)
            }
        }
        union_proto.set_fields(proto_fields);
        stat_proto.set_name("Union".to_string());
        stat_proto.set_union(union_proto);
    }
    Some(stat_proto)
}


/*
A few utilities to visit types and statistics
 */
fn table_structs<'a>(
    t: &'a type_::Type,
    s: Option<&'a statistics::Statistics>,
) -> Vec<(
    Identifier,
    &'a type_::type_::Struct,
    Option<&'a statistics::statistics::Struct>,
)> {
    if let Some(t) = t.type_.as_ref() {
        match t {
            type_::type_::Type::Struct(t) => {
                // If the type is a Struct
                let s = s.and_then(|s| s.statistics.as_ref()).and_then(|s| match s {
                    statistics::statistics::Statistics::Struct(s) => Some(s),
                    _ => None,
                });
                vec![(Identifier::empty(), t, s)]
            }
            type_::type_::Type::Union(t) => {
                // If the type is a Union
                let s = s.and_then(|s| s.statistics.as_ref()).and_then(|s| match s {
                    statistics::statistics::Statistics::Union(s) => Some(s),
                    _ => None,
                });
                t.fields()
                    .iter()
                    .flat_map(|f| {
                        let g = s.and_then(|s| {
                            s.fields().iter().find_map(|g| {
                                if g.name() == f.name() {
                                    Some(g.statistics())
                                } else {
                                    None
                                }
                            })
                        });
                        table_structs(f.type_(), g)
                            .into_iter()
                            .map(|(i, t, s)| (i.with((0, f.name().to_string())), t, s))
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    }
}


/// Builds a DataType from a protobuf Type
impl<'a> From<&'a type_::Type> for DataType {
    fn from(value: &'a type_::Type) -> Self {
        value.type_.as_ref().map_or(DataType::Any, |t| match t {
            type_::type_::Type::Null(type_::type_::Null { .. }) => DataType::Null,
            type_::type_::Type::Unit(type_::type_::Unit { .. }) => DataType::unit(),
            type_::type_::Type::Boolean(type_::type_::Boolean { .. }) => {
                DataType::boolean()
            }
            type_::type_::Type::Integer(type_::type_::Integer {
                min,
                max,
                possible_values,
                ..
            }) => {
                if possible_values.len() > 0 {
                    DataType::integer_values(possible_values)
                } else {
                    DataType::integer_interval(*min, *max)
                }
            }
            type_::type_::Type::Enum(type_::type_::Enum { name_values, .. }) => DataType::Enum(
                name_values
                    .iter()
                    .map(|nv| (nv.name(), nv.value()))
                    .collect(),
            ),
            type_::type_::Type::Float(type_::type_::Float {
                min,
                max,
                possible_values,
                ..
            }) => {
                if possible_values.len() > 0 {
                    DataType::float_values(possible_values)
                } else {
                    DataType::float_interval(*min, *max)
                }
            }
            type_::type_::Type::Text(type_::type_::Text {
                possible_values, ..
            }) => {
                if possible_values.len() > 0 {
                    DataType::text_values(possible_values)
                } else {
                    DataType::text()
                }
            }
            type_::type_::Type::Bytes(type_::type_::Bytes { special_fields }) => DataType::bytes(),
            type_::type_::Type::Struct(type_::type_::Struct {
                fields,
                ..
            }) => DataType::Struct(data_type::Struct::new(
                fields
                    .iter()
                    .map(|f| (f.name().to_string(), Arc::new(f.type_().into())))
                    .collect(),
            )),
            type_::type_::Type::Union(type_::type_::Union {
                fields,
                ..
            }) => DataType::Union(data_type::Union::new(
                fields
                    .iter()
                    .map(|f| (f.name().to_string(), Arc::new(f.type_().into())))
                    .collect(),
            )),
            type_::type_::Type::Optional(type_::type_::Optional { type_, .. }) => {
                DataType::optional(type_.get_or_default().into())
            }
            type_::type_::Type::List(type_::type_::List {
                type_,
                max_size,
                ..
            }) => DataType::list(type_.get_or_default().into(), 0, *max_size as usize),
            type_::type_::Type::Array(type_::type_::Array {
                type_,
                shape,
                ..
            }) => DataType::Array(data_type::Array::new(
                Arc::new(type_.get_or_default().into()),
                shape.iter().map(|x| *x as usize).collect(),
            )),
            type_::type_::Type::Date(type_::type_::Date {
                format,
                min,
                max,
                possible_values,
                base,
                ..
            }) => {
                if possible_values.len() > 0 {
                    let possible_dates: Result<Vec<NaiveDate>> = possible_values
                        .iter()
                        .map(|d| Ok(NaiveDate::parse_from_str(d, format)?))
                        .collect();
                    possible_dates.map_or_else(
                        |e| DataType::Any,
                        |possible_dates| DataType::date_values(possible_dates),
                    )
                } else {
                    NaiveDate::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveDate::parse_from_str(max, format)?)))
                        .map_or_else(
                            |e| DataType::Any,
                            |(min, max)| DataType::date_interval(min, max),
                        )
                }
            }
            type_::type_::Type::Time(type_::type_::Time {
                format,
                min,
                max,
                possible_values,
                base,
                ..
            }) => {
                if possible_values.len() > 0 {
                    let possible_times: Result<Vec<NaiveTime>> = possible_values
                        .iter()
                        .map(|d| Ok(NaiveTime::parse_from_str(d, format)?))
                        .collect();
                    possible_times.map_or_else(
                        |e| DataType::Any,
                        |possible_times| DataType::time_values(possible_times),
                    )
                } else {
                    NaiveTime::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveTime::parse_from_str(max, format)?)))
                        .map_or_else(
                            |e| DataType::Any,
                            |(min, max)| DataType::time_interval(min, max),
                        )
                }
            }
            type_::type_::Type::Datetime(type_::type_::Datetime {
                format,
                min,
                max,
                possible_values,
                base,
                ..
            }) => {
                if possible_values.len() > 0 {
                    let possible_date_times: Result<Vec<NaiveDateTime>> = possible_values
                        .iter()
                        .map(|d| Ok(NaiveDateTime::parse_from_str(d, format)?))
                        .collect();
                    possible_date_times.map_or_else(
                        |e| DataType::Any,
                        |possible_date_times| DataType::date_time_values(possible_date_times),
                    )
                } else {
                    NaiveDateTime::parse_from_str(min, format)
                        .and_then(|min| Ok((min, NaiveDateTime::parse_from_str(max, format)?)))
                        .map_or_else(
                            |e| DataType::Any,
                            |(min, max)| DataType::date_time_interval(min, max),
                        )
                }
            }
            type_::type_::Type::Duration(type_::type_::Duration {
                unit,
                min,
                max,
                possible_values,
                ..
            }) => {
                let format_duration = match unit.as_str() {
                    "ns" => Duration::nanoseconds,
                    "us" => Duration::microseconds,
                    "ms" => Duration::milliseconds,
                    "s" => Duration::seconds,
                    _ => panic!("stop"),
                };
                if possible_values.len() > 0 {
                    let possible_date_times: Result<Vec<Duration>> = possible_values
                        .iter()
                        .map(|d| Ok(format_duration(*d)))
                        .collect();
                    possible_date_times
                        .map_or_else(|e| DataType::Any, |d| DataType::duration_values(d))
                } else {
                    DataType::duration_interval(format_duration(*min), format_duration(*max))
                }
            }
            type_::type_::Type::Id(type_::type_::Id {
                unique, reference, ..
            }) => DataType::Id(data_type::Id::new(None, *unique)),
            _ => DataType::Any,
        })
    }
}

/// Builds a Protobuf Type out of a Sarus DataType
impl <'a> TryFrom<&'a DataType> for type_::Type {
    type Error = Error;

    fn try_from(data_type: &DataType) -> Result<type_::Type> {
        let mut proto_type = type_::Type::new();
        match data_type {
            DataType::Null => {
                proto_type.set_name("Null".to_string());
                proto_type.set_null(type_::type_::Null::new());
            }
            DataType::Unit(_) => {
                proto_type.set_name("Unit".to_string());
                proto_type.set_unit(type_::type_::Unit::new());
            }
            DataType::Boolean(_) => {
                proto_type.set_name("Boolean".to_string());
                proto_type.set_boolean(type_::type_::Boolean::new());
            }
            DataType::Integer(integer) => {
                let mut integer_type = type_::type_::Integer::new();
                if let Some(m) = integer.min() {
                    integer_type.set_min(*m);
                }
                if let Some(m) = integer.max() {
                    integer_type.set_max(*m);
                }
                if integer.all_values() {
                    integer_type
                        .set_possible_values(integer.iter().map(|[min, _]| min.clone()).collect());
                }
                proto_type.set_name("Integer".to_string());
                proto_type.set_integer(integer_type);
            }
            DataType::Enum(enum_) => {
                let mut enum_type = type_::type_::Enum::new();
                let enum_values: Vec<type_::type_::enum_::NameValue> = enum_
                    .values()
                    .iter()
                    .map(|(v, i)| {
                        let mut enum_val = type_::type_::enum_::NameValue::new();
                        enum_val.set_name(v.clone());
                        enum_val.set_value(*i);
                        enum_val
                    })
                    .collect();
                enum_type.set_name_values(enum_values);

                proto_type.set_name("Enum".to_string());
                proto_type.set_enum(enum_type);
            }
            DataType::Float(float) => {
                let mut float_type = type_::type_::Float::new();
                if let Some(m) = float.min() {
                    float_type.set_min(*m);
                }
                if let Some(m) = float.max() {
                    float_type.set_max(*m);
                }
                if float.all_values() {
                    float_type
                        .set_possible_values(float.iter().map(|[min, _]| min.clone()).collect());
                }
                proto_type.set_name("Float".to_string());
                proto_type.set_float(float_type);
            }
            DataType::Text(text) => {
                let mut text_type = type_::type_::Text::new();
                if text.all_values() {
                    text_type
                        .set_possible_values(text.iter().map(|[min, _]| min.clone()).collect());
                }
                //text_type.set_encoding("UTF8".to_string());

                proto_type.set_name("Text".to_string());
                proto_type.set_text(text_type);
            }
            DataType::Bytes(_) => {
                proto_type.set_name("Bytes".to_string());
                proto_type.set_bytes(type_::type_::Bytes::new());
            }
            DataType::Struct(struct_type_) => {
                let mut struct_type = type_::type_::Struct::new();
                let mut proto_fields: Vec<type_::type_::struct_::Field> = vec![];
                for (name, dtype) in struct_type_.fields() {
                    let mut data_field = type_::type_::struct_::Field::new();
                    data_field.set_name(name.to_string());
                    data_field.set_type(dtype.as_ref().try_into()?);
                    proto_fields.push(data_field)
                }
                struct_type.set_fields(proto_fields);

                proto_type.set_name("Struct".to_string());
                proto_type.set_struct(struct_type);
            }
            DataType::Union(union) => {
                let mut union_type = type_::type_::Union::new();
                let mut proto_fields: Vec<type_::type_::union::Field> = vec![];
                for (name, dtype) in union.fields() {
                    let mut data_field = type_::type_::union::Field::new();
                    data_field.set_name(name.to_string());
                    data_field.set_type(dtype.as_ref().try_into()?);
                    proto_fields.push(data_field)
                }
                union_type.set_fields(proto_fields);

                proto_type.set_name("Union".to_string());
                proto_type.set_union(union_type);
            }
            DataType::Optional(optional) => {
                let mut optional_type = type_::type_::Optional::new();
                let data_type: type_::Type = optional.data_type().try_into()?;
                optional_type.set_type(data_type);

                proto_type.set_name("Optional".to_string());
                proto_type.set_optional(optional_type);
            }
            DataType::List(list_) => {
                let mut list_type = type_::type_::List::new();
                let data_type: type_::Type = list_.data_type().try_into()?;
                list_type.set_type(data_type);
                if let Some(number) = list_.size().max() {
                    list_type.set_max_size(*number);
                }

                proto_type.set_name("List".to_string());
                proto_type.set_list(list_type);
            }
            DataType::Set(_set) => {
                return Err(Error::Other(
                    "Cannot convert DataType::Set to protobuf::_type::Type".to_string(),
                ))
            }
            DataType::Array(array) => {
                let mut array_type = type_::type_::Array::new();
                let data_type: type_::Type = array.data_type().try_into()?;
                array_type.set_type(data_type);
                let mut shape: Vec<i64> = vec![];
                for s in array.shape() {
                    match s.clone().try_into() {
                        Ok(conv_s) => shape.push(conv_s),
                        Err(_) => {
                            return Err(Error::Other(
                                "Cannot convert shape from usize to i64".to_string(),
                            ))
                        }
                    }
                }
                array_type.set_shape(shape);

                proto_type.set_name("Array".to_string());
                proto_type.set_array(array_type);
            }
            DataType::Date(date) => {
                let mut date_type = type_::type_::Date::new();
                let format = "%Y-%m-%d";
                date_type.set_format(format.to_string());
                if let Some(m) = date.min() {
                    date_type.set_min(m.format(format).to_string());
                }
                if let Some(m) = date.max() {
                    date_type.set_max(m.format(format).to_string());
                }
                if date.all_values() {
                    date_type.set_possible_values(
                        date.iter()
                            .map(|[min, _]| min.format(format).to_string())
                            .collect(),
                    );
                }
                proto_type.set_name("Date".to_string());
                proto_type.set_date(date_type);
            }
            DataType::Time(time) => {
                let mut time_type = type_::type_::Time::new();
                let format = "%H:%M:%S.%9f";
                time_type.set_format(format.to_string());
                time_type.set_base(type_::type_::time::Base::INT64_NS);
                if let Some(m) = time.min() {
                    time_type.set_min(m.format(format).to_string());
                }
                if let Some(m) = time.max() {
                    time_type.set_max(m.format(format).to_string());
                }
                if time.all_values() {
                    time_type.set_possible_values(
                        time.iter()
                            .map(|[min, _]| min.format(format).to_string())
                            .collect(),
                    );
                }
                proto_type.set_name("Time".to_string());
                proto_type.set_time(time_type);
            }
            DataType::DateTime(date_time) => {
                let mut date_time_type = type_::type_::Datetime::new();
                let format = "%Y-%m-%d %H:%M:%S.%9f";
                date_time_type.set_format(format.to_string());
                if let Some(m) = date_time.min() {
                    date_time_type.set_min(m.format(format).to_string());
                }
                if let Some(m) = date_time.max() {
                    date_time_type.set_max(m.format(format).to_string());
                }
                if date_time.all_values() {
                    date_time_type.set_possible_values(
                        date_time
                            .iter()
                            .map(|[min, _]| min.format(format).to_string())
                            .collect(),
                    );
                }
                proto_type.set_name("Datetime".to_string());
                proto_type.set_datetime(date_time_type);
            }
            DataType::Duration(duration) => {
                let mut duration_type = type_::type_::Duration::new();

                let mut vec_of_durations: Vec<Duration> =
                    duration.iter().map(|[min, _]| min.clone()).collect();
                if let Some(m) = duration.min() {
                    vec_of_durations.push(m.clone());
                }
                if let Some(m) = duration.max() {
                    vec_of_durations.push(m.clone())
                }

                let (duration_unit, conversion) = match vec_of_durations.iter().max() {
                    Some(m) => {
                        if m.num_nanoseconds().is_some() {
                            (
                                "ns",
                                Box::new(|dur: &Duration| dur.num_nanoseconds().unwrap())
                                as Box<dyn Fn(&Duration) -> i64>,
                            )
                        } else if m.num_microseconds().is_some() {
                            (
                                "us",
                                Box::new(|dur: &Duration| dur.num_microseconds().unwrap())
                                as Box<dyn Fn(&Duration) -> i64>,
                            )
                        } else {
                            (
                                "ms",
                                Box::new(|dur: &Duration| dur.num_milliseconds())
                                as Box<dyn Fn(&Duration) -> i64>,
                            )
                        }
                    },
                    None => {
                        return Err(Error::Other(
                            "Cannot infert Duration unit if min, max or possible values are not provided".to_string()
                        ))
                    }
                };

                duration_type.set_unit(duration_unit.to_string());
                if let Some(m) = duration.min() {
                    duration_type.set_min(conversion(m))
                }

                if let Some(m) = duration.max() {
                    duration_type.set_max(conversion(m))
                }

                if duration.all_values() {
                    duration_type.set_possible_values(
                        duration.iter().map(|[min, _]| conversion(min)).collect(),
                    );
                }

                proto_type.set_name("Duration".to_string());
                proto_type.set_duration(duration_type);
            }
            DataType::Id(id) => {
                let mut id_type = type_::type_::Id::new();
                id_type.set_unique(id.unique());

                proto_type.set_name("Id".to_string());
                proto_type.set_id(type_::type_::Id::new());
            }
            DataType::Function(_function) => {
                return Err(Error::Other(
                    "Cannot convert DataType::Function to protobuf::_type::Type".to_string(),
                ))
            }
            DataType::Any => {
                return Err(Error::Other(
                    "Cannot convert DataType::Any to protobuf::_type::Type".to_string(),
                ))
            }
        };
        Ok(proto_type)
    }
}

/// Builds a Table Schema out of a Sarus Struct
impl<'a> From<&'a type_::type_::Struct> for Schema {
    fn from(t: &'a type_::type_::Struct) -> Self {
        t.fields()
            .iter()
            .map(|f| (
                f.name(),
                DataType::from(f.type_()),
                f.type_().properties().get(CONSTRAINT).and_then(|constraint| if constraint==CONSTRAINT_UNIQUE {Some(Constraint::Unique)} else {None})
            ))
            .collect()
    }
}

fn relation_from_struct<'a>(
    identifier: Identifier,
    schema_struct: &'a type_::type_::Struct,
    size_struct: Option<&'a statistics::statistics::Struct>,
) -> Relation {
    let schema: Schema = schema_struct.try_into().unwrap();
    let mut builder = Relation::table().schema(schema);
    // Create a table builder with a name
    builder = builder.path(identifier);
    if let Some(s) = size_struct {
        builder = builder.size(s.size())
    }
    builder.build()
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use qrlew::{relation::Table};

    fn relation() -> Relation {
        let schema: Schema = vec![
            ("a", DataType::integer_interval(-1, 1), None),
            ("b", DataType::float_interval(-2., 2.), Some(Constraint::Unique)),
        ]
        .into_iter()
        .collect();
        let tab: Relation = Table::builder().schema(schema).size(200).build();
        tab
    }

    #[test]
    fn test_relations_empty_path() -> Result<()> {
        let empty_path: Vec<String> = vec![];
        let relations = Hierarchy::from([
            (empty_path, Arc::new(relation())),
        ]);
        assert!(Dataset::try_from(&relations).is_err());
        Ok(())
    }

    #[test]
    fn test_relations_multiple_path_head() -> Result<()> {
        let relations = Hierarchy::from([
            (vec!["a"], Arc::new(relation())),
            (vec!["e"], Arc::new(relation()))
        ]);
        assert!(Dataset::try_from(&relations).is_err());
        Ok(())
    }

    #[test]
    fn test_relations_single_entity_path() -> Result<()> {
        let tab_as_relation = relation();
        let rel_schema = tab_as_relation.schema();
        let mut schema_type: type_::Type = (&rel_schema.data_type()).try_into()?;
        // Pretty ugly but it works
        schema_type.mut_struct().fields[1].mut_type().mut_properties().insert(CONSTRAINT.into(), CONSTRAINT_UNIQUE.into());
        
        let relations = Hierarchy::from([
            (vec!["my_table"], Arc::new(tab_as_relation.clone())),
        ]);
        let ds = Dataset::try_from(&relations)?;

        let ds_schema_proto = ds.schema();
        let ds_size_proto = ds.size();
        
        if let Some(proto) = ds_size_proto {
            assert!(proto.statistics().struct_().size() == 200);
        };
        assert!(ds_schema_proto.name() == "my_table");
        assert!(ds.schema_type_data() == &schema_type);
        let schema_str = r#"
            {
                "name": "my_table",
                "type": {
                    "name": "Struct",
                    "struct": {
                        "fields": [{
                            "name": "sarus_data",
                            "type": {
                                "name": "Struct",
                                "struct": {
                                    "fields": [{
                                        "name": "a",
                                        "type": {
                                            "name": "Integer",
                                            "integer": {
                                                "min": "-1",
                                                "max": "1"
                                            }
                                        }
                                    }, {
                                        "name": "b",
                                        "type": {
                                            "name": "Float",
                                            "float": {
                                                "min": -2.0,
                                                "max": 2.0
                                            },
                                            "properties": {"_CONSTRAINT_": "_UNIQUE_"}
                                        }
                                    }]
                                }
                            }
                        }, {
                            "name": "sarus_protected_entity",
                            "type": {
                                "name": "Optional",
                                "optional": {
                                    "type": {
                                        "name": "Id",
                                        "id": {}
                                    }
                                }
                            }
                        }, {
                            "name": "sarus_is_public",
                            "type": {
                                "name": "Boolean",
                                "boolean": {}
                            }
                        }, {
                            "name": "sarus_weights",
                            "type": {
                                "name": "Float",
                                "float": {
                                    "max": 1.7976931348623157e308
                                }
                            }
                        }]
                    }
                }
            }
        "#;
        let parsed_schema: schema::Schema = parse_from_str(schema_str).unwrap();
        assert!(&parsed_schema==ds_schema_proto);
        Ok(())
    } 

    #[test]
    fn test_relations_unions_case_1() -> Result<()> {
        let tab_as_relation = relation();
        let relations = Hierarchy::from([
            (vec!["a", "b"], Arc::new(tab_as_relation.clone())),
        ]);
        
        let ds = Dataset::try_from(&relations)?;

        let ds_schema_proto = ds.schema();
        let ds_size_proto = ds.size();

        if let Some(proto) = ds_size_proto {
            println!("STATS: \n{}\n", print_to_string(proto).unwrap());
            let tab_size = proto.statistics().union().fields().as_ref()[0].statistics().struct_().size();
            assert!(tab_size == 200);
        };
        assert!(ds_schema_proto.name() == "a");
        let schema_str = r#"
        {
            "name": "a",
            "type": {
              "name": "Struct",
              "struct": {
                "fields": [{
                  "name": "sarus_data",
                  "type": {
                    "name": "Union",
                    "union": {
                      "fields": [{
                        "name": "b",
                        "type": {
                          "name": "Struct",
                          "struct": {
                            "fields": [{
                              "name": "a",
                              "type": {
                                "name": "Integer",
                                "integer": {
                                  "min": "-1",
                                  "max": "1"
                                }
                              }
                            }, {
                              "name": "b",
                              "type": {
                                "name": "Float",
                                "float": {
                                  "min": -2.0,
                                  "max": 2.0
                                },
                                "properties": {"_CONSTRAINT_": "_UNIQUE_"}
                              }
                            }]
                          }
                        }
                      }]
                    }
                  }
                }, {
                  "name": "sarus_protected_entity",
                  "type": {
                    "name": "Optional",
                    "optional": {
                      "type": {
                        "name": "Id",
                        "id": {}
                      }
                    }
                  }
                }, {
                  "name": "sarus_is_public",
                  "type": {
                    "name": "Boolean",
                    "boolean": {}
                  }
                }, {
                  "name": "sarus_weights",
                  "type": {
                    "name": "Float",
                    "float": {
                      "max": 1.7976931348623157e308
                    }
                  }
                }]
              }
            }
          }
        "#;
        let parsed_schema: schema::Schema = parse_from_str(schema_str).unwrap();
        assert!(&parsed_schema==ds_schema_proto);
        Ok(())
    }

    #[test]
    fn test_relations_unions_case_2() -> Result<()> {
        let tab_as_relation = relation();

        let relations = Hierarchy::from([
            (vec!["a", "b"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "c"], Arc::new(tab_as_relation.clone())),
        ]);

        let ds = Dataset::try_from(&relations)?;
        let ds_schema_proto = ds.schema();
        let ds_size_proto = ds.size();

        if let Some(proto) = ds_size_proto {
            println!("STATS: \n{}\n", print_to_string(proto).unwrap());
            assert!(proto.statistics().union().fields().as_ref()[0].statistics().struct_().size() == 200);
            assert!(proto.statistics().union().fields().as_ref()[1].statistics().struct_().size() == 200);
        };
        assert!(ds_schema_proto.name() == "a");
        let schema_str = r#"
        {
            "name": "a",
            "type": {
              "name": "Struct",
              "struct": {
                "fields": [{
                  "name": "sarus_data",
                  "type": {
                    "name": "Union",
                    "union": {
                      "fields": [{
                        "name": "c",
                        "type": {
                          "name": "Struct",
                          "struct": {
                            "fields": [{
                              "name": "a",
                              "type": {
                                "name": "Integer",
                                "integer": {
                                  "min": "-1",
                                  "max": "1"
                                }
                              }
                            }, {
                              "name": "b",
                              "type": {
                                "name": "Float",
                                "float": {
                                  "min": -2.0,
                                  "max": 2.0
                                }
                              }
                            }]
                          }
                        }
                      }, {
                        "name": "b",
                        "type": {
                          "name": "Struct",
                          "struct": {
                            "fields": [{
                              "name": "a",
                              "type": {
                                "name": "Integer",
                                "integer": {
                                  "min": "-1",
                                  "max": "1"
                                }
                              }
                            }, {
                              "name": "b",
                              "type": {
                                "name": "Float",
                                "float": {
                                  "min": -2.0,
                                  "max": 2.0
                                }
                              }
                            }]
                          }
                        }
                      }]
                    }
                  }
                }, {
                  "name": "sarus_protected_entity",
                  "type": {
                    "name": "Optional",
                    "optional": {
                      "type": {
                        "name": "Id",
                        "id": {}
                      }
                    }
                  }
                }, {
                  "name": "sarus_is_public",
                  "type": {
                    "name": "Boolean",
                    "boolean": {}
                  }
                }, {
                  "name": "sarus_weights",
                  "type": {
                    "name": "Float",
                    "float": {
                      "max": 1.7976931348623157e308
                    }
                  }
                }]
              }
            }
          }
        "#;
        let parsed_schema: schema::Schema = parse_from_str(schema_str).unwrap();
        let parsed_type: DataType = parsed_schema.type_().try_into().unwrap();
        let ds_type: DataType = ds_schema_proto.type_().try_into().unwrap();
        assert!(ds_type==parsed_type);
        Ok(())
    }

    #[test]
    fn test_relations_unions_case_3() -> Result<()> {
        let tab_as_relation = relation();

        let relations = Hierarchy::from([
            (vec!["a", "b", "d"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "b", "e", "f"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "c"], Arc::new(tab_as_relation.clone())),
        ]);

        let ds = Dataset::try_from(&relations)?;
        let ds_schema_proto = ds.schema();
        let ds_size_proto = ds.size();
        assert!(ds_schema_proto.name() == "a");
        
        if let Some(proto) = ds_size_proto {
            println!("STATS: \n{}\n", print_to_string(proto).unwrap());
        };

        let schema_str = r#"
        {
            "name": "a",
            "type": {
              "name": "Struct",
              "struct": {
                "fields": [{
                  "name": "sarus_data",
                  "type": {
                    "name": "Union",
                    "union": {
                      "fields": [{
                        "name": "c",
                        "type": {
                          "name": "Struct",
                          "struct": {
                            "fields": [{
                              "name": "a",
                              "type": {
                                "name": "Integer",
                                "integer": {
                                  "min": "-1",
                                  "max": "1"
                                }
                              }
                            }, {
                              "name": "b",
                              "type": {
                                "name": "Float",
                                "float": {
                                  "min": -2.0,
                                  "max": 2.0
                                }
                              }
                            }]
                          }
                        }
                      }, {
                        "name": "b",
                        "type": {
                          "name": "Union",
                          "union": {
                            "fields": [{
                              "name": "d",
                              "type": {
                                "name": "Struct",
                                "struct": {
                                  "fields": [{
                                    "name": "a",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "min": "-1",
                                        "max": "1"
                                      }
                                    }
                                  }, {
                                    "name": "b",
                                    "type": {
                                      "name": "Float",
                                      "float": {
                                        "min": -2.0,
                                        "max": 2.0
                                      }
                                    }
                                  }]
                                }
                              }
                            }, {
                              "name": "e",
                              "type": {
                                "name": "Union",
                                "union": {
                                  "fields": [{
                                    "name": "f",
                                    "type": {
                                      "name": "Struct",
                                      "struct": {
                                        "fields": [{
                                          "name": "a",
                                          "type": {
                                            "name": "Integer",
                                            "integer": {
                                              "min": "-1",
                                              "max": "1"
                                            }
                                          }
                                        }, {
                                          "name": "b",
                                          "type": {
                                            "name": "Float",
                                            "float": {
                                              "min": -2.0,
                                              "max": 2.0
                                            }
                                          }
                                        }]
                                      }
                                    }
                                  }]
                                }
                              }
                            }]
                          }
                        }
                      }]
                    }
                  }
                }, {
                  "name": "sarus_protected_entity",
                  "type": {
                    "name": "Optional",
                    "optional": {
                      "type": {
                        "name": "Id",
                        "id": {}
                      }
                    }
                  }
                }, {
                  "name": "sarus_is_public",
                  "type": {
                    "name": "Boolean",
                    "boolean": {}
                  }
                }, {
                  "name": "sarus_weights",
                  "type": {
                    "name": "Float",
                    "float": {
                      "max": 1.7976931348623157e308
                    }
                  }
                }]
              }
            }
          }
        "#;
        let parsed_schema: schema::Schema = parse_from_str(schema_str).unwrap();
        let parsed_type: DataType = parsed_schema.type_().try_into().unwrap();
        let ds_type: DataType = ds_schema_proto.type_().try_into().unwrap();
        assert!(ds_type==parsed_type);
        Ok(())
    }

    #[test]
    fn test_null() -> Result<()> {
        let type_str: &str = r#"
           {
            "@type": "sarus_data_spec/sarus_data_spec.Type",
            "name": "Null",
            "properties": {},
            "null": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str).unwrap();
        let sarus_type = DataType::from(&proto_data_type);
        assert!(sarus_type == DataType::Null);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);

        Ok(())
    }

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);
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
        assert!(sarus_type == DataType::Boolean(data_type::Boolean::default()));
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);
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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);
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

        let my_rc_vec: Arc<[(String, i64)]> = Arc::from(my_vec);
        assert!(sarus_type == DataType::Enum(data_type::Enum::new(my_rc_vec)));
        println!("{:?}", sarus_type);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type.enum_().name_values() == new_proto_data_type.enum_().name_values());
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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.float().min() == 0.0);
        assert!(new_proto_data_type.float().max() == 5.0);
        assert!(new_proto_data_type.float().possible_values() == &[0., 1., 5.]);

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.float().min() == 0.0);
        assert!(new_proto_data_type.float().max() == 1.0);
        assert!(new_proto_data_type.float().possible_values() == &[]);
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
        assert!(
            sarus_type
                == DataType::text_values(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert_eq!(new_proto_data_type.name(), "Text".to_string());
        assert_eq!(new_proto_data_type.text().encoding(), "".to_string());
        assert_eq!(
            new_proto_data_type.text().possible_values(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert_eq!(new_proto_data_type.name(), "Text".to_string());
        assert_eq!(new_proto_data_type.text().encoding(), "".to_string());
        assert!(new_proto_data_type.text().possible_values().is_empty());

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);
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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.date().min() == "2000-01-01");
        assert!(new_proto_data_type.date().max() == "2100-01-01");
        assert!(new_proto_data_type.date().possible_values().is_empty());

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
        let ok_results = DataType::date_values(vec![
            NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d")?,
            NaiveDate::parse_from_str("2001-01-01", "%Y-%m-%d")?,
            NaiveDate::parse_from_str("2100-01-01", "%Y-%m-%d")?,
        ]);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.date().min() == "2000-01-01");
        assert!(new_proto_data_type.date().max() == "2100-01-01");
        assert!(new_proto_data_type.date().possible_values().len() == 3);
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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.time().min() == "12:12:01.000000000");
        assert!(new_proto_data_type.time().max() == "12:12:03.000000000");
        assert!(format!("{:?}", new_proto_data_type.time().base()) == "INT64_NS".to_string());
        assert!(new_proto_data_type.time().possible_values().is_empty());

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
        let ok_results = DataType::time_values(vec![
            NaiveTime::parse_from_str("12:12:01.000000", "%H:%M:%S.%f")?,
            NaiveTime::parse_from_str("12:12:02.000000", "%H:%M:%S.%f")?,
            NaiveTime::parse_from_str("12:12:03.000000", "%H:%M:%S.%f")?,
        ]);
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.time().min() == "12:12:01.000000000");
        assert!(new_proto_data_type.time().max() == "12:12:03.000000000");
        assert!(new_proto_data_type.time().possible_values().len() == 3);
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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.datetime().min() == "2023-01-01 00:00:00.000000000");
        assert!(new_proto_data_type.datetime().max() == "2023-12-31 00:00:00.000000000");
        assert!(new_proto_data_type.datetime().possible_values().is_empty());
        assert!(format!("{:?}", new_proto_data_type.datetime().base()) == "INT64_NS".to_string());

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
        let ok_results = DataType::date_time_values(vec![
            NaiveDateTime::parse_from_str("2023-01-01 00:10:00", "%Y-%m-%d %H:%M:%S")?,
            NaiveDateTime::parse_from_str("2023-06-01 00:00:30", "%Y-%m-%d %H:%M:%S")?,
            NaiveDateTime::parse_from_str("2023-12-01 11:00:00", "%Y-%m-%d %H:%M:%S")?,
        ]);
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.datetime().min() == "2023-01-01 00:10:00.000000000");
        assert!(new_proto_data_type.datetime().max() == "2023-12-01 11:00:00.000000000");
        assert!(new_proto_data_type.datetime().possible_values().len() == 3);
        assert!(format!("{:?}", new_proto_data_type.datetime().base()) == "INT64_NS".to_string());

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.duration().min() == 1234567000);
        assert!(new_proto_data_type.duration().max() == 3234567000);
        assert!(new_proto_data_type.duration().possible_values().is_empty());
        assert!(new_proto_data_type.duration().unit() == "ns");

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
        let ok_results = DataType::duration_values(vec![
            Duration::microseconds(1234567),
            Duration::microseconds(2234567),
            Duration::microseconds(3234567),
        ]);
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(new_proto_data_type.duration().min() == 1234567000);
        assert!(new_proto_data_type.duration().max() == 3234567000);
        assert!(
            new_proto_data_type.duration().possible_values()
                == [1234567000 as i64, 2234567000 as i64, 3234567000 as i64].as_slice()
        );
        assert!(new_proto_data_type.duration().unit() == "ns");

        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "duration": {
                "max": "3234567000000",
                "min": "123456700000",
                "possible_values": [
                    "123456700000",
                    "3234567000000"
                ],
                "unit": "s"
                },
                "name": "Duration",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        println!("sarus_type: {:?}", sarus_type);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        //println!("new_proto_data_type: {:#?}", new_proto_data_type);
        assert!(new_proto_data_type.duration().min() == 123456700000000000);
        assert!(new_proto_data_type.duration().max() == 3234567000000000000);
        assert!(
            new_proto_data_type.duration().possible_values()
                == [123456700000000000 as i64, 3234567000000000000 as i64].as_slice()
        );
        assert!(new_proto_data_type.duration().unit() == "us");

        let type_str: &str = r#"
            {
                "@type": "sarus_data_spec/sarus_data_spec.Type",
                "duration": {
                "max": "3234567000000000",
                "min": "123456700000000",
                "possible_values": [
                    "123456700000000",
                    "3234567000000000"
                ],
                "unit": "s"
                },
                "name": "Duration",
                "properties": {}
            }
        "#;
        let proto_data_type: type_::Type = parse_from_str(type_str)?;
        let sarus_type = DataType::from(&proto_data_type);
        println!("sarus_type: {:?}", sarus_type);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        println!("{:#?}", new_proto_data_type);
        assert!(new_proto_data_type.duration().min() == 123456700000000000);
        assert!(new_proto_data_type.duration().max() == 3234567000000000000);
        assert!(
            new_proto_data_type.duration().possible_values()
                == [123456700000000000 as i64, 3234567000000000000 as i64].as_slice()
        );
        assert!(new_proto_data_type.duration().unit() == "ms");

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
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type.id().unique() == new_proto_data_type.id().unique());

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
            (
                "integer_possible_values".to_string(),
                Arc::new(DataType::integer_interval(
                    -9223372036854775808,
                    9223372036854775807,
                )),
            ),
            (
                "text".to_string(),
                Arc::new(DataType::text_values(vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                ])),
            ),
        ]));
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        println!("\nnew: {:?}", new_proto_data_type.struct_().fields());
        assert_eq!(new_proto_data_type.struct_().fields().len(), 2);
        assert_eq!(
            new_proto_data_type.struct_().fields()[0].name(),
            "integer_possible_values".to_string()
        );
        assert_eq!(
            new_proto_data_type.struct_().fields()[0]
                .type_()
                .integer()
                .min(),
            -9223372036854775808
        );
        assert_eq!(
            new_proto_data_type.struct_().fields()[0]
                .type_()
                .integer()
                .max(),
            9223372036854775807
        );
        assert_eq!(
            new_proto_data_type.struct_().fields()[1].name(),
            "text".to_string()
        );
        assert_eq!(
            new_proto_data_type.struct_().fields()[1]
                .type_()
                .text()
                .possible_values(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
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
            (
                "integer_possible_values".to_string(),
                Arc::new(DataType::integer_interval(
                    -9223372036854775808,
                    9223372036854775807,
                )),
            ),
            (
                "text".to_string(),
                Arc::new(DataType::text_values(vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                ])),
            ),
        ]));
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        println!("\nold: {:?}", proto_data_type);
        println!("\nnew: {:?}", new_proto_data_type);
        assert_eq!(new_proto_data_type.union().fields().len(), 2);
        assert_eq!(
            new_proto_data_type.union().fields()[0].name(),
            "integer_possible_values".to_string()
        );
        assert_eq!(
            new_proto_data_type.union().fields()[0]
                .type_()
                .integer()
                .min(),
            -9223372036854775808
        );
        assert_eq!(
            new_proto_data_type.union().fields()[0]
                .type_()
                .integer()
                .max(),
            9223372036854775807
        );
        assert_eq!(
            new_proto_data_type.union().fields()[1].name(),
            "text".to_string()
        );
        assert_eq!(
            new_proto_data_type.union().fields()[1]
                .type_()
                .text()
                .possible_values(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );

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
        let ok_results = DataType::optional(DataType::integer_interval(
            -9223372036854775808,
            9223372036854775807,
        ));
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);

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
            DataType::integer_interval(-9223372036854775808, 9223372036854775807),
            0,
            5,
        );
        println!("{:?}", sarus_type);
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type == new_proto_data_type);

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
            Arc::new(DataType::integer_interval(
                -9223372036854775808,
                9223372036854775807,
            )),
            Arc::new([1]),
        ));
        println!("{:?}", ok_results);
        assert!(sarus_type == ok_results);
        let new_proto_data_type: type_::Type = (&sarus_type).try_into()?;
        assert!(proto_data_type.id().unique() == new_proto_data_type.id().unique());
        assert!(proto_data_type == new_proto_data_type);

        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_range() -> Result<()> {
        let tab_as_relation = relation();
        let relations = Hierarchy::from([
            (vec!["a", "b", "d"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "b", "e", "f"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "c"], Arc::new(tab_as_relation.clone())),
        ]);
        let ds = Dataset::try_from(&relations)?;
        let ds = ds.with_range("b", "d", "b", -101., 101.)?;
        assert!(ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().float().min()==-101.);
        Ok(())
    }

    #[test]
    fn test_constraint() -> Result<()> {
        let tab_as_relation = relation();
        let relations = Hierarchy::from([
            (vec!["a", "b", "d"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "b", "e", "f"], Arc::new(tab_as_relation.clone())),
            (vec!["a", "c"], Arc::new(tab_as_relation.clone())),
        ]);
        let ds = Dataset::try_from(&relations)?;
        println!("{:?}", ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="a").unwrap()
            .type_().properties());
        assert!(ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="a").unwrap()
            .type_().properties().get(CONSTRAINT)==None);
        let ds = ds.with_constraint("b", "d", "a", Some(CONSTRAINT_UNIQUE))?;
        println!("{:?}", ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="a").unwrap()
            .type_().properties());
        assert!(ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="a").unwrap()
            .type_().properties().get(CONSTRAINT).unwrap()==CONSTRAINT_UNIQUE);
        let ds = ds.with_constraint("b", "d", "a", None)?;
        assert!(ds.schema()
            .type_().struct_().fields().iter().find(|f| f.name()=="sarus_data").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="b").unwrap()
            .type_().union().fields().iter().find(|f| f.name()=="d").unwrap()
            .type_().struct_().fields().iter().find(|f| f.name()=="a").unwrap()
            .type_().properties().get(CONSTRAINT)==None);
        Ok(())
    }
}
