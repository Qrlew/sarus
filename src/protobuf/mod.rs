//! Generated from SDS v2.11
pub mod attribute;
pub mod bounds;
pub mod constraint;
pub mod dataset;
pub mod links;
pub mod manager;
pub mod marginals;
pub mod path;
pub mod predicate;
pub mod proto_container;
pub mod relation;
pub mod scalar;
pub mod schema;
pub mod size;
pub mod statistics;
pub mod status;
pub mod transform;
pub mod type_;

use protobuf::MessageFull;
use protobuf_json_mapping::{parse_from_str_with_options, ParseOptions};
pub use protobuf_json_mapping::{print_to_string, ParseError, PrintError};

pub fn parse_from_str<M: MessageFull>(json: &str) -> Result<M, ParseError> {
    parse_from_str_with_options(
        json,
        &ParseOptions {
            ignore_unknown_fields: true,
            ..Default::default()
        },
    )
}
