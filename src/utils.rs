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