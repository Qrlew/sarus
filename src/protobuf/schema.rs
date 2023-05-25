// This file is generated by rust-protobuf 3.2.0. Do not edit
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

//! Generated file from `sarus_data_spec/protobuf/schema.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:sarus_data_spec.Schema)
pub struct Schema {
    // message fields
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.uuid)
    pub uuid: ::std::string::String,
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.dataset)
    pub dataset: ::std::string::String,
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.type)
    pub type_: ::protobuf::MessageField<super::type_::Type>,
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.protected)
    pub protected: ::protobuf::MessageField<super::path::Path>,
    // @@protoc_insertion_point(field:sarus_data_spec.Schema.properties)
    pub properties: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:sarus_data_spec.Schema.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Schema {
    fn default() -> &'a Schema {
        <Schema as ::protobuf::Message>::default_instance()
    }
}

impl Schema {
    pub fn new() -> Schema {
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

    // string dataset = 2;

    pub fn dataset(&self) -> &str {
        &self.dataset
    }

    pub fn clear_dataset(&mut self) {
        self.dataset.clear();
    }

    // Param is passed by value, moved
    pub fn set_dataset(&mut self, v: ::std::string::String) {
        self.dataset = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataset(&mut self) -> &mut ::std::string::String {
        &mut self.dataset
    }

    // Take field
    pub fn take_dataset(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dataset, ::std::string::String::new())
    }

    // string name = 3;

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .sarus_data_spec.Type type = 4;

    pub fn type_(&self) -> &super::type_::Type {
        self.type_.as_ref().unwrap_or_else(|| <super::type_::Type as ::protobuf::Message>::default_instance())
    }

    pub fn clear_type_(&mut self) {
        self.type_.clear();
    }

    pub fn has_type(&self) -> bool {
        self.type_.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type(&mut self, v: super::type_::Type) {
        self.type_ = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type(&mut self) -> &mut super::type_::Type {
        self.type_.mut_or_insert_default()
    }

    // Take field
    pub fn take_type_(&mut self) -> super::type_::Type {
        self.type_.take().unwrap_or_else(|| super::type_::Type::new())
    }

    // .sarus_data_spec.Path protected = 5;

    pub fn protected(&self) -> &super::path::Path {
        self.protected.as_ref().unwrap_or_else(|| <super::path::Path as ::protobuf::Message>::default_instance())
    }

    pub fn clear_protected(&mut self) {
        self.protected.clear();
    }

    pub fn has_protected(&self) -> bool {
        self.protected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protected(&mut self, v: super::path::Path) {
        self.protected = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protected(&mut self) -> &mut super::path::Path {
        self.protected.mut_or_insert_default()
    }

    // Take field
    pub fn take_protected(&mut self) -> super::path::Path {
        self.protected.take().unwrap_or_else(|| super::path::Path::new())
    }

    // repeated .sarus_data_spec.Schema.PropertiesEntry properties = 6;

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
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uuid",
            |m: &Schema| { &m.uuid },
            |m: &mut Schema| { &mut m.uuid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dataset",
            |m: &Schema| { &m.dataset },
            |m: &mut Schema| { &mut m.dataset },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Schema| { &m.name },
            |m: &mut Schema| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::type_::Type>(
            "type",
            |m: &Schema| { &m.type_ },
            |m: &mut Schema| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::path::Path>(
            "protected",
            |m: &Schema| { &m.protected },
            |m: &mut Schema| { &mut m.protected },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "properties",
            |m: &Schema| { &m.properties },
            |m: &mut Schema| { &mut m.properties },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Schema>(
            "Schema",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Schema {
    const NAME: &'static str = "Schema";

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
                    self.dataset = is.read_string()?;
                },
                26 => {
                    self.name = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.type_)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.protected)?;
                },
                50 => {
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
        if !self.dataset.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.dataset);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if let Some(v) = self.type_.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.protected.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
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
        if !self.dataset.is_empty() {
            os.write_string(2, &self.dataset)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if let Some(v) = self.type_.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.protected.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        for (k, v) in &self.properties {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(50)?; // Tag.
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

    fn new() -> Schema {
        Schema::new()
    }

    fn clear(&mut self) {
        self.uuid.clear();
        self.dataset.clear();
        self.name.clear();
        self.type_.clear();
        self.protected.clear();
        self.properties.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Schema {
        static instance: ::protobuf::rt::Lazy<Schema> = ::protobuf::rt::Lazy::new();
        instance.get(Schema::new)
    }
}

impl ::protobuf::MessageFull for Schema {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Schema").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Schema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Schema {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Schema`
pub mod schema {
    #[derive(PartialEq,Clone,Default,Debug)]
    // @@protoc_insertion_point(message:sarus_data_spec.Schema.Hypothesis)
    pub struct Hypothesis {
        // message fields
        // @@protoc_insertion_point(field:sarus_data_spec.Schema.Hypothesis.uuid)
        pub uuid: ::std::string::String,
        // @@protoc_insertion_point(field:sarus_data_spec.Schema.Hypothesis.dataset)
        pub dataset: ::std::string::String,
        // @@protoc_insertion_point(field:sarus_data_spec.Schema.Hypothesis.name)
        pub name: ::std::string::String,
        // @@protoc_insertion_point(field:sarus_data_spec.Schema.Hypothesis.type)
        pub type_: ::protobuf::MessageField<super::super::type_::Type>,
        // @@protoc_insertion_point(field:sarus_data_spec.Schema.Hypothesis.properties)
        pub properties: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        // special fields
        // @@protoc_insertion_point(special_field:sarus_data_spec.Schema.Hypothesis.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Hypothesis {
        fn default() -> &'a Hypothesis {
            <Hypothesis as ::protobuf::Message>::default_instance()
        }
    }

    impl Hypothesis {
        pub fn new() -> Hypothesis {
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

        // string dataset = 2;

        pub fn dataset(&self) -> &str {
            &self.dataset
        }

        pub fn clear_dataset(&mut self) {
            self.dataset.clear();
        }

        // Param is passed by value, moved
        pub fn set_dataset(&mut self, v: ::std::string::String) {
            self.dataset = v;
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_dataset(&mut self) -> &mut ::std::string::String {
            &mut self.dataset
        }

        // Take field
        pub fn take_dataset(&mut self) -> ::std::string::String {
            ::std::mem::replace(&mut self.dataset, ::std::string::String::new())
        }

        // string name = 3;

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn clear_name(&mut self) {
            self.name.clear();
        }

        // Param is passed by value, moved
        pub fn set_name(&mut self, v: ::std::string::String) {
            self.name = v;
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_name(&mut self) -> &mut ::std::string::String {
            &mut self.name
        }

        // Take field
        pub fn take_name(&mut self) -> ::std::string::String {
            ::std::mem::replace(&mut self.name, ::std::string::String::new())
        }

        // .sarus_data_spec.Type type = 4;

        pub fn type_(&self) -> &super::super::type_::Type {
            self.type_.as_ref().unwrap_or_else(|| <super::super::type_::Type as ::protobuf::Message>::default_instance())
        }

        pub fn clear_type_(&mut self) {
            self.type_.clear();
        }

        pub fn has_type(&self) -> bool {
            self.type_.is_some()
        }

        // Param is passed by value, moved
        pub fn set_type(&mut self, v: super::super::type_::Type) {
            self.type_ = ::protobuf::MessageField::some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_type(&mut self) -> &mut super::super::type_::Type {
            self.type_.mut_or_insert_default()
        }

        // Take field
        pub fn take_type_(&mut self) -> super::super::type_::Type {
            self.type_.take().unwrap_or_else(|| super::super::type_::Type::new())
        }

        // repeated .sarus_data_spec.Schema.Hypothesis.PropertiesEntry properties = 5;

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

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(5);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "uuid",
                |m: &Hypothesis| { &m.uuid },
                |m: &mut Hypothesis| { &mut m.uuid },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "dataset",
                |m: &Hypothesis| { &m.dataset },
                |m: &mut Hypothesis| { &mut m.dataset },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "name",
                |m: &Hypothesis| { &m.name },
                |m: &mut Hypothesis| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::type_::Type>(
                "type",
                |m: &Hypothesis| { &m.type_ },
                |m: &mut Hypothesis| { &mut m.type_ },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
                "properties",
                |m: &Hypothesis| { &m.properties },
                |m: &mut Hypothesis| { &mut m.properties },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Hypothesis>(
                "Schema.Hypothesis",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Hypothesis {
        const NAME: &'static str = "Hypothesis";

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
                        self.dataset = is.read_string()?;
                    },
                    26 => {
                        self.name = is.read_string()?;
                    },
                    34 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.type_)?;
                    },
                    42 => {
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
            if !self.dataset.is_empty() {
                my_size += ::protobuf::rt::string_size(2, &self.dataset);
            }
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(3, &self.name);
            }
            if let Some(v) = self.type_.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
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
            if !self.dataset.is_empty() {
                os.write_string(2, &self.dataset)?;
            }
            if !self.name.is_empty() {
                os.write_string(3, &self.name)?;
            }
            if let Some(v) = self.type_.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
            }
            for (k, v) in &self.properties {
                let mut entry_size = 0;
                entry_size += ::protobuf::rt::string_size(1, &k);
                entry_size += ::protobuf::rt::string_size(2, &v);
                os.write_raw_varint32(42)?; // Tag.
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

        fn new() -> Hypothesis {
            Hypothesis::new()
        }

        fn clear(&mut self) {
            self.uuid.clear();
            self.dataset.clear();
            self.name.clear();
            self.type_.clear();
            self.properties.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Hypothesis {
            static instance: ::protobuf::rt::Lazy<Hypothesis> = ::protobuf::rt::Lazy::new();
            instance.get(Hypothesis::new)
        }
    }

    impl ::protobuf::MessageFull for Hypothesis {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Schema.Hypothesis").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Hypothesis {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Hypothesis {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%sarus_data_spec/protobuf/schema.proto\x12\x0fsarus_data_spec\x1a#saru\
    s_data_spec/protobuf/type.proto\x1a#sarus_data_spec/protobuf/path.proto\
    \"\xc1\x04\n\x06Schema\x12\x12\n\x04uuid\x18\x01\x20\x01(\tR\x04uuid\x12\
    \x18\n\x07dataset\x18\x02\x20\x01(\tR\x07dataset\x12\x12\n\x04name\x18\
    \x03\x20\x01(\tR\x04name\x12)\n\x04type\x18\x04\x20\x01(\x0b2\x15.sarus_\
    data_spec.TypeR\x04type\x123\n\tprotected\x18\x05\x20\x01(\x0b2\x15.saru\
    s_data_spec.PathR\tprotected\x12G\n\nproperties\x18\x06\x20\x03(\x0b2'.s\
    arus_data_spec.Schema.PropertiesEntryR\nproperties\x1a=\n\x0fPropertiesE\
    ntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value:\x028\x01\x1a\x8c\x02\n\nHypothesis\x12\x12\n\
    \x04uuid\x18\x01\x20\x01(\tR\x04uuid\x12\x18\n\x07dataset\x18\x02\x20\
    \x01(\tR\x07dataset\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12)\n\
    \x04type\x18\x04\x20\x01(\x0b2\x15.sarus_data_spec.TypeR\x04type\x12R\n\
    \nproperties\x18\x05\x20\x03(\x0b22.sarus_data_spec.Schema.Hypothesis.Pr\
    opertiesEntryR\nproperties\x1a=\n\x0fPropertiesEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05va\
    lue:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::type_::file_descriptor().clone());
            deps.push(super::path::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Schema::generated_message_descriptor_data());
            messages.push(schema::Hypothesis::generated_message_descriptor_data());
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
