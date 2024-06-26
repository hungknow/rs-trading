// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc 25.3
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

//! Generated file from `add_event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AddEventRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddEventRequest {
    // message fields
    // @@protoc_insertion_point(field:AddEventRequest.a)
    pub a: i64,
    // @@protoc_insertion_point(field:AddEventRequest.b)
    pub b: i64,
    // special fields
    // @@protoc_insertion_point(special_field:AddEventRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddEventRequest {
    fn default() -> &'a AddEventRequest {
        <AddEventRequest as ::protobuf::Message>::default_instance()
    }
}

impl AddEventRequest {
    pub fn new() -> AddEventRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "a",
            |m: &AddEventRequest| { &m.a },
            |m: &mut AddEventRequest| { &mut m.a },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "b",
            |m: &AddEventRequest| { &m.b },
            |m: &mut AddEventRequest| { &mut m.b },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddEventRequest>(
            "AddEventRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddEventRequest {
    const NAME: &'static str = "AddEventRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.a = is.read_int64()?;
                },
                16 => {
                    self.b = is.read_int64()?;
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
        if self.a != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.a);
        }
        if self.b != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.b);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.a != 0 {
            os.write_int64(1, self.a)?;
        }
        if self.b != 0 {
            os.write_int64(2, self.b)?;
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

    fn new() -> AddEventRequest {
        AddEventRequest::new()
    }

    fn clear(&mut self) {
        self.a = 0;
        self.b = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddEventRequest {
        static instance: AddEventRequest = AddEventRequest {
            a: 0,
            b: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddEventRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddEventRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddEventRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddEventRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:AddEventResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddEventResponse {
    // message fields
    // @@protoc_insertion_point(field:AddEventResponse.result)
    pub result: i64,
    // special fields
    // @@protoc_insertion_point(special_field:AddEventResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddEventResponse {
    fn default() -> &'a AddEventResponse {
        <AddEventResponse as ::protobuf::Message>::default_instance()
    }
}

impl AddEventResponse {
    pub fn new() -> AddEventResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "result",
            |m: &AddEventResponse| { &m.result },
            |m: &mut AddEventResponse| { &mut m.result },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddEventResponse>(
            "AddEventResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddEventResponse {
    const NAME: &'static str = "AddEventResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.result = is.read_int64()?;
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
        if self.result != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.result);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.result != 0 {
            os.write_int64(1, self.result)?;
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

    fn new() -> AddEventResponse {
        AddEventResponse::new()
    }

    fn clear(&mut self) {
        self.result = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddEventResponse {
        static instance: AddEventResponse = AddEventResponse {
            result: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddEventResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddEventResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddEventResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddEventResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fadd_event.proto\"-\n\x0fAddEventRequest\x12\x0c\n\x01a\x18\x01\x20\
    \x01(\x03R\x01a\x12\x0c\n\x01b\x18\x02\x20\x01(\x03R\x01b\"*\n\x10AddEve\
    ntResponse\x12\x16\n\x06result\x18\x01\x20\x01(\x03R\x06resultb\x06proto\
    3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(AddEventRequest::generated_message_descriptor_data());
            messages.push(AddEventResponse::generated_message_descriptor_data());
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
