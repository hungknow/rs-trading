use crate::protos::ffi_request::FFIRequest;
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
impl_protobuf_message_to_vec!(FFIRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf_message_to_vec() {
        let mut request = FFIRequest::new();
        request.event = "test".to_string();
        request.payload = vec![1, 2, 3, 4];

        let bytes: Vec<u8> = request.into();
        let parsed_request = FFIRequest::parse_from_bytes(&bytes).unwrap();

        assert_eq!(parsed_request.event, "test");
        assert_eq!(parsed_request.payload, vec![1, 2, 3, 4]);
    }
}
