// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `steammessages_market.steamclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct CEconMarket_IsMarketplaceAllowed_Request {
    // message fields
    webcookie: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CEconMarket_IsMarketplaceAllowed_Request {
    fn default() -> &'a CEconMarket_IsMarketplaceAllowed_Request {
        <CEconMarket_IsMarketplaceAllowed_Request as ::protobuf::Message>::default_instance()
    }
}

impl CEconMarket_IsMarketplaceAllowed_Request {
    pub fn new() -> CEconMarket_IsMarketplaceAllowed_Request {
        ::std::default::Default::default()
    }

    // optional string webcookie = 1;


    pub fn get_webcookie(&self) -> &str {
        match self.webcookie.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_webcookie(&mut self) {
        self.webcookie.clear();
    }

    pub fn has_webcookie(&self) -> bool {
        self.webcookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webcookie(&mut self, v: ::std::string::String) {
        self.webcookie = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_webcookie(&mut self) -> &mut ::std::string::String {
        if self.webcookie.is_none() {
            self.webcookie.set_default();
        }
        self.webcookie.as_mut().unwrap()
    }

    // Take field
    pub fn take_webcookie(&mut self) -> ::std::string::String {
        self.webcookie.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for CEconMarket_IsMarketplaceAllowed_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.webcookie)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.webcookie.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.webcookie.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CEconMarket_IsMarketplaceAllowed_Request {
        CEconMarket_IsMarketplaceAllowed_Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "webcookie",
                |m: &CEconMarket_IsMarketplaceAllowed_Request| { &m.webcookie },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Request| { &mut m.webcookie },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CEconMarket_IsMarketplaceAllowed_Request>(
                "CEconMarket_IsMarketplaceAllowed_Request",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CEconMarket_IsMarketplaceAllowed_Request {
        static instance: ::protobuf::rt::LazyV2<CEconMarket_IsMarketplaceAllowed_Request> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CEconMarket_IsMarketplaceAllowed_Request::new)
    }
}

impl ::protobuf::Clear for CEconMarket_IsMarketplaceAllowed_Request {
    fn clear(&mut self) {
        self.webcookie.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEconMarket_IsMarketplaceAllowed_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEconMarket_IsMarketplaceAllowed_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEconMarket_IsMarketplaceAllowed_Response {
    // message fields
    allowed: ::std::option::Option<bool>,
    reason: ::std::option::Option<u32>,
    allowed_at_time: ::std::option::Option<u32>,
    steamguard_required_days: ::std::option::Option<u32>,
    forms_requested: ::std::option::Option<bool>,
    forms_require_verification: ::std::option::Option<bool>,
    new_device_cooldown_days: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CEconMarket_IsMarketplaceAllowed_Response {
    fn default() -> &'a CEconMarket_IsMarketplaceAllowed_Response {
        <CEconMarket_IsMarketplaceAllowed_Response as ::protobuf::Message>::default_instance()
    }
}

impl CEconMarket_IsMarketplaceAllowed_Response {
    pub fn new() -> CEconMarket_IsMarketplaceAllowed_Response {
        ::std::default::Default::default()
    }

    // optional bool allowed = 1;


    pub fn get_allowed(&self) -> bool {
        self.allowed.unwrap_or(false)
    }
    pub fn clear_allowed(&mut self) {
        self.allowed = ::std::option::Option::None;
    }

    pub fn has_allowed(&self) -> bool {
        self.allowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed(&mut self, v: bool) {
        self.allowed = ::std::option::Option::Some(v);
    }

    // optional uint32 reason = 2;


    pub fn get_reason(&self) -> u32 {
        self.reason.unwrap_or(0)
    }
    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: u32) {
        self.reason = ::std::option::Option::Some(v);
    }

    // optional uint32 allowed_at_time = 3;


    pub fn get_allowed_at_time(&self) -> u32 {
        self.allowed_at_time.unwrap_or(0)
    }
    pub fn clear_allowed_at_time(&mut self) {
        self.allowed_at_time = ::std::option::Option::None;
    }

    pub fn has_allowed_at_time(&self) -> bool {
        self.allowed_at_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_at_time(&mut self, v: u32) {
        self.allowed_at_time = ::std::option::Option::Some(v);
    }

    // optional uint32 steamguard_required_days = 4;


    pub fn get_steamguard_required_days(&self) -> u32 {
        self.steamguard_required_days.unwrap_or(0)
    }
    pub fn clear_steamguard_required_days(&mut self) {
        self.steamguard_required_days = ::std::option::Option::None;
    }

    pub fn has_steamguard_required_days(&self) -> bool {
        self.steamguard_required_days.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamguard_required_days(&mut self, v: u32) {
        self.steamguard_required_days = ::std::option::Option::Some(v);
    }

    // optional bool forms_requested = 7;


    pub fn get_forms_requested(&self) -> bool {
        self.forms_requested.unwrap_or(false)
    }
    pub fn clear_forms_requested(&mut self) {
        self.forms_requested = ::std::option::Option::None;
    }

    pub fn has_forms_requested(&self) -> bool {
        self.forms_requested.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forms_requested(&mut self, v: bool) {
        self.forms_requested = ::std::option::Option::Some(v);
    }

    // optional bool forms_require_verification = 8;


    pub fn get_forms_require_verification(&self) -> bool {
        self.forms_require_verification.unwrap_or(false)
    }
    pub fn clear_forms_require_verification(&mut self) {
        self.forms_require_verification = ::std::option::Option::None;
    }

    pub fn has_forms_require_verification(&self) -> bool {
        self.forms_require_verification.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forms_require_verification(&mut self, v: bool) {
        self.forms_require_verification = ::std::option::Option::Some(v);
    }

    // optional uint32 new_device_cooldown_days = 9;


    pub fn get_new_device_cooldown_days(&self) -> u32 {
        self.new_device_cooldown_days.unwrap_or(0)
    }
    pub fn clear_new_device_cooldown_days(&mut self) {
        self.new_device_cooldown_days = ::std::option::Option::None;
    }

    pub fn has_new_device_cooldown_days(&self) -> bool {
        self.new_device_cooldown_days.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_device_cooldown_days(&mut self, v: u32) {
        self.new_device_cooldown_days = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for CEconMarket_IsMarketplaceAllowed_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allowed = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.reason = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.allowed_at_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.steamguard_required_days = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.forms_requested = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.forms_require_verification = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.new_device_cooldown_days = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.allowed {
            my_size += 2;
        }
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.allowed_at_time {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.steamguard_required_days {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.forms_requested {
            my_size += 2;
        }
        if let Some(v) = self.forms_require_verification {
            my_size += 2;
        }
        if let Some(v) = self.new_device_cooldown_days {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.allowed {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.reason {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.allowed_at_time {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.steamguard_required_days {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.forms_requested {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.forms_require_verification {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.new_device_cooldown_days {
            os.write_uint32(9, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CEconMarket_IsMarketplaceAllowed_Response {
        CEconMarket_IsMarketplaceAllowed_Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "allowed",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.allowed },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.allowed },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "reason",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.reason },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.reason },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "allowed_at_time",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.allowed_at_time },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.allowed_at_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "steamguard_required_days",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.steamguard_required_days },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.steamguard_required_days },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "forms_requested",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.forms_requested },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.forms_requested },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "forms_require_verification",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.forms_require_verification },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.forms_require_verification },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "new_device_cooldown_days",
                |m: &CEconMarket_IsMarketplaceAllowed_Response| { &m.new_device_cooldown_days },
                |m: &mut CEconMarket_IsMarketplaceAllowed_Response| { &mut m.new_device_cooldown_days },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CEconMarket_IsMarketplaceAllowed_Response>(
                "CEconMarket_IsMarketplaceAllowed_Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CEconMarket_IsMarketplaceAllowed_Response {
        static instance: ::protobuf::rt::LazyV2<CEconMarket_IsMarketplaceAllowed_Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CEconMarket_IsMarketplaceAllowed_Response::new)
    }
}

impl ::protobuf::Clear for CEconMarket_IsMarketplaceAllowed_Response {
    fn clear(&mut self) {
        self.allowed = ::std::option::Option::None;
        self.reason = ::std::option::Option::None;
        self.allowed_at_time = ::std::option::Option::None;
        self.steamguard_required_days = ::std::option::Option::None;
        self.forms_requested = ::std::option::Option::None;
        self.forms_require_verification = ::std::option::Option::None;
        self.new_device_cooldown_days = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEconMarket_IsMarketplaceAllowed_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEconMarket_IsMarketplaceAllowed_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&steammessages_market.steamclient.proto\x1a,steammessages_unified_base\
    .steamclient.proto\"y\n(CEconMarket_IsMarketplaceAllowed_Request\x12M\n\
    \twebcookie\x18\x01\x20\x01(\tR\twebcookieB/\x82\xb5\x18+The\x20user's\
    \x20Steam\x20Guard\x20machine\x20auth\x20cookie.\"\xef\x06\n)CEconMarket\
    _IsMarketplaceAllowed_Response\x12R\n\x07allowed\x18\x01\x20\x01(\x08R\
    \x07allowedB8\x82\xb5\x184Whether\x20or\x20not\x20the\x20user\x20is\x20a\
    llowed\x20to\x20use\x20the\x20market\x12S\n\x06reason\x18\x02\x20\x01(\r\
    R\x06reasonB;\x82\xb5\x187The\x20reason\x20the\x20user\x20can't\x20use\
    \x20the\x20market,\x20if\x20applicable\x12_\n\x0fallowed_at_time\x18\x03\
    \x20\x01(\rR\rallowedAtTimeB7\x82\xb5\x183The\x20time\x20the\x20user\x20\
    will\x20be\x20allowed\x20to\x20use\x20the\x20market\x12\x9c\x01\n\x18ste\
    amguard_required_days\x18\x04\x20\x01(\rR\x16steamguardRequiredDaysBb\
    \x82\xb5\x18^The\x20number\x20of\x20days\x20any\x20user\x20is\x20require\
    d\x20to\x20have\x20had\x20Steam\x20Guard\x20before\x20they\x20can\x20use\
    \x20the\x20market\x12g\n\x0fforms_requested\x18\x07\x20\x01(\x08R\x0efor\
    msRequestedB>\x82\xb5\x18:Whether\x20or\x20not\x20we've\x20requested\x20\
    the\x20user\x20fill\x20out\x20tax\x20forms\x12\x82\x01\n\x1aforms_requir\
    e_verification\x18\x08\x20\x01(\x08R\x18formsRequireVerificationBD\x82\
    \xb5\x18@True\x20if\x20we've\x20received\x20forms\x20but\x20they\x20requ\
    ire\x20verification\x20first\x12\xaa\x01\n\x18new_device_cooldown_days\
    \x18\t\x20\x01(\rR\x15newDeviceCooldownDaysBq\x82\xb5\x18mThe\x20number\
    \x20of\x20days\x20after\x20initial\x20device\x20authorization\x20a\x20us\
    er\x20must\x20wait\x20before\x20using\x20the\x20market\x20on\x20that\x20\
    device2\xee\x01\n\nEconMarket\x12\xb8\x01\n\x14IsMarketplaceAllowed\x12)\
    .CEconMarket_IsMarketplaceAllowed_Request\x1a*.CEconMarket_IsMarketplace\
    Allowed_Response\"I\x82\xb5\x18EChecks\x20whether\x20or\x20not\x20the\
    \x20authed\x20account\x20is\x20allowed\x20to\x20use\x20the\x20market\x1a\
    %\x82\xb5\x18!A\x20service\x20to\x20use\x20market\x20functionsB\x03\x80\
    \x01\x01\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
