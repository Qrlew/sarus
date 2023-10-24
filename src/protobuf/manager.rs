// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `sarus_data_spec/protobuf/manager.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:sarus_data_spec.Manager)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Manager {
    // message fields
    // @@protoc_insertion_point(field:sarus_data_spec.Manager.uuid)
    pub uuid: ::std::string::String,
    // @@protoc_insertion_point(field:sarus_data_spec.Manager.properties)
    pub properties: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:sarus_data_spec.Manager.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Manager {
    fn default() -> &'a Manager {
        <Manager as ::protobuf::Message>::default_instance()
    }
}

impl Manager {
    pub fn new() -> Manager {
        ::std::default::Default::default()
    }

    // string uuid = 1;

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::string::String) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::string::String {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uuid, ::std::string::String::new())
    }

    // repeated .sarus_data_spec.Manager.PropertiesEntry properties = 2;

    pub fn properties(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.properties
    }

    pub fn clear_properties(&mut self) {
        self.properties.clear();
    }

    // Param is passed by value, moved
    pub fn set_properties(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.properties = v;
    }

    // Mutable pointer to the field.
    pub fn mut_properties(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.properties
    }

    // Take field
    pub fn take_properties(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.properties, ::std::collections::HashMap::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uuid",
            |m: &Manager| { &m.uuid },
            |m: &mut Manager| { &mut m.uuid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "properties",
            |m: &Manager| { &m.properties },
            |m: &mut Manager| { &mut m.properties },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Manager>(
            "Manager",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Manager {
    const NAME: &'static str = "Manager";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.uuid = is.read_string()?;
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.properties.insert(key, value);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.uuid);
        }
        for (k, v) in &self.properties {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.uuid.is_empty() {
            os.write_string(1, &self.uuid)?;
        }
        for (k, v) in &self.properties {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Manager {
        Manager::new()
    }

    fn clear(&mut self) {
        self.uuid.clear();
        self.properties.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Manager {
        static instance: ::protobuf::rt::Lazy<Manager> = ::protobuf::rt::Lazy::new();
        instance.get(Manager::new)
    }
}

impl ::protobuf::MessageFull for Manager {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Manager").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Manager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Manager {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&sarus_data_spec/protobuf/manager.proto\x12\x0fsarus_data_spec\"\xa6\
    \x01\n\x07Manager\x12\x12\n\x04uuid\x18\x01\x20\x01(\tR\x04uuid\x12H\n\n\
    properties\x18\x02\x20\x03(\x0b2(.sarus_data_spec.Manager.PropertiesEntr\
    yR\nproperties\x1a=\n\x0fPropertiesEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\
    \x01b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Manager::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
