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

//! Generated file from `brand.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:brand_info)
pub struct Brand_info {
    // message fields
    // @@protoc_insertion_point(field:brand_info.brand_id)
    pub brand_id: ::std::string::String,
    // @@protoc_insertion_point(field:brand_info.brand_name)
    pub brand_name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:brand_info.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Brand_info {
    fn default() -> &'a Brand_info {
        <Brand_info as ::protobuf::Message>::default_instance()
    }
}

impl Brand_info {
    pub fn new() -> Brand_info {
        ::std::default::Default::default()
    }

    // string brand_id = 1;

    pub fn brand_id(&self) -> &str {
        &self.brand_id
    }

    pub fn clear_brand_id(&mut self) {
        self.brand_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_brand_id(&mut self, v: ::std::string::String) {
        self.brand_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_brand_id(&mut self) -> &mut ::std::string::String {
        &mut self.brand_id
    }

    // Take field
    pub fn take_brand_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.brand_id, ::std::string::String::new())
    }

    // string brand_name = 2;

    pub fn brand_name(&self) -> &str {
        &self.brand_name
    }

    pub fn clear_brand_name(&mut self) {
        self.brand_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_brand_name(&mut self, v: ::std::string::String) {
        self.brand_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_brand_name(&mut self) -> &mut ::std::string::String {
        &mut self.brand_name
    }

    // Take field
    pub fn take_brand_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.brand_name, ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "brand_id",
            |m: &Brand_info| { &m.brand_id },
            |m: &mut Brand_info| { &mut m.brand_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "brand_name",
            |m: &Brand_info| { &m.brand_name },
            |m: &mut Brand_info| { &mut m.brand_name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Brand_info>(
            "brand_info",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Brand_info {
    const NAME: &'static str = "brand_info";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.brand_id = is.read_string()?;
                },
                18 => {
                    self.brand_name = is.read_string()?;
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
        if !self.brand_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.brand_id);
        }
        if !self.brand_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.brand_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.brand_id.is_empty() {
            os.write_string(1, &self.brand_id)?;
        }
        if !self.brand_name.is_empty() {
            os.write_string(2, &self.brand_name)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Brand_info {
        Brand_info::new()
    }

    fn clear(&mut self) {
        self.brand_id.clear();
        self.brand_name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Brand_info {
        static instance: Brand_info = Brand_info {
            brand_id: ::std::string::String::new(),
            brand_name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Brand_info {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("brand_info").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Brand_info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Brand_info {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:brand_info_list)
pub struct Brand_info_list {
    // message fields
    // @@protoc_insertion_point(field:brand_info_list.brands)
    pub brands: ::std::vec::Vec<Brand_info>,
    // special fields
    // @@protoc_insertion_point(special_field:brand_info_list.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Brand_info_list {
    fn default() -> &'a Brand_info_list {
        <Brand_info_list as ::protobuf::Message>::default_instance()
    }
}

impl Brand_info_list {
    pub fn new() -> Brand_info_list {
        ::std::default::Default::default()
    }

    // repeated .brand_info brands = 1;

    pub fn brands(&self) -> &[Brand_info] {
        &self.brands
    }

    pub fn clear_brands(&mut self) {
        self.brands.clear();
    }

    // Param is passed by value, moved
    pub fn set_brands(&mut self, v: ::std::vec::Vec<Brand_info>) {
        self.brands = v;
    }

    // Mutable pointer to the field.
    pub fn mut_brands(&mut self) -> &mut ::std::vec::Vec<Brand_info> {
        &mut self.brands
    }

    // Take field
    pub fn take_brands(&mut self) -> ::std::vec::Vec<Brand_info> {
        ::std::mem::replace(&mut self.brands, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "brands",
            |m: &Brand_info_list| { &m.brands },
            |m: &mut Brand_info_list| { &mut m.brands },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Brand_info_list>(
            "brand_info_list",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Brand_info_list {
    const NAME: &'static str = "brand_info_list";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.brands.push(is.read_message()?);
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
        for value in &self.brands {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.brands {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> Brand_info_list {
        Brand_info_list::new()
    }

    fn clear(&mut self) {
        self.brands.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Brand_info_list {
        static instance: Brand_info_list = Brand_info_list {
            brands: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Brand_info_list {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("brand_info_list").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Brand_info_list {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Brand_info_list {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:brand_add)
pub struct Brand_add {
    // message fields
    // @@protoc_insertion_point(field:brand_add.brand)
    pub brand: ::protobuf::MessageField<Brand_info>,
    // special fields
    // @@protoc_insertion_point(special_field:brand_add.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Brand_add {
    fn default() -> &'a Brand_add {
        <Brand_add as ::protobuf::Message>::default_instance()
    }
}

impl Brand_add {
    pub fn new() -> Brand_add {
        ::std::default::Default::default()
    }

    // .brand_info brand = 1;

    pub fn brand(&self) -> &Brand_info {
        self.brand.as_ref().unwrap_or_else(|| <Brand_info as ::protobuf::Message>::default_instance())
    }

    pub fn clear_brand(&mut self) {
        self.brand.clear();
    }

    pub fn has_brand(&self) -> bool {
        self.brand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_brand(&mut self, v: Brand_info) {
        self.brand = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_brand(&mut self) -> &mut Brand_info {
        self.brand.mut_or_insert_default()
    }

    // Take field
    pub fn take_brand(&mut self) -> Brand_info {
        self.brand.take().unwrap_or_else(|| Brand_info::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Brand_info>(
            "brand",
            |m: &Brand_add| { &m.brand },
            |m: &mut Brand_add| { &mut m.brand },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Brand_add>(
            "brand_add",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Brand_add {
    const NAME: &'static str = "brand_add";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.brand)?;
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
        if let Some(v) = self.brand.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.brand.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Brand_add {
        Brand_add::new()
    }

    fn clear(&mut self) {
        self.brand.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Brand_add {
        static instance: Brand_add = Brand_add {
            brand: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Brand_add {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("brand_add").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Brand_add {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Brand_add {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bbrand.proto\"F\n\nbrand_info\x12\x19\n\x08brand_id\x18\x01\x20\x01\
    (\tR\x07brandId\x12\x1d\n\nbrand_name\x18\x02\x20\x01(\tR\tbrandName\"6\
    \n\x0fbrand_info_list\x12#\n\x06brands\x18\x01\x20\x03(\x0b2\x0b.brand_i\
    nfoR\x06brands\".\n\tbrand_add\x12!\n\x05brand\x18\x01\x20\x01(\x0b2\x0b\
    .brand_infoR\x05brandb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(Brand_info::generated_message_descriptor_data());
            messages.push(Brand_info_list::generated_message_descriptor_data());
            messages.push(Brand_add::generated_message_descriptor_data());
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
