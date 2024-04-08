use crate::protos::ffi_request::HkFFIRequest;
use protobuf::Message;

// #[warn(unused_macros)]
macro_rules! impl_protobuf_message_to_vec {
    ($t: ident) => {
        impl From<$t> for Vec<u8> {
            fn from(message: $t) -> Self {
                let mut buf = Vec::new();
                message.write_to_vec(&mut buf).unwrap();
                buf
            }
        }
    };
}

// Convert from Protobuf message to array of bytes
// impl<T: ::protobuf::Message> From<T> for Vec<u8> {
//     fn from(message: T) -> Self {
//         let mut buf = Vec::new();
//         message.write_to_vec(&mut buf).unwrap();
//         buf
//     }
// }
impl_protobuf_message_to_vec!(HkFFIRequest);

#[cfg(test)]
mod tests {
    use crate::protos::ffi_event_type::HkFFIEventType;

    use super::*;

    #[test]
    fn test_protobuf_message_to_vec() {
        let mut request = HkFFIRequest::new();
        request.event = HkFFIEventType::HK_FFI_REQ_UNKNOWN.into();
        request.payload = vec![1, 2, 3, 4];

        let bytes: Vec<u8> = request.into();
        let parsed_request = HkFFIRequest::parse_from_bytes(&bytes).unwrap();

        assert_eq!(
            parsed_request.event,
            HkFFIEventType::HK_FFI_REQ_UNKNOWN.into()
        );
        assert_eq!(parsed_request.payload, vec![1, 2, 3, 4]);
    }
}
